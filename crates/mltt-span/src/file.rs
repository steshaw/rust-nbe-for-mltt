use std::ops;

use crate::{ByteIndex, ColumnIndex, LineIndex, Location, Span};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(usize);

impl FileId {
    pub fn to_usize(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct File {
    id: FileId,
    name: String,
    contents: String,
}

impl File {
    pub fn id(&self) -> FileId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn span(&self) -> Span<FileId> {
        Span::from_str(self.id(), self.contents())
    }
}

#[derive(Debug, Clone)]
pub struct Files {
    files: Vec<File>,
}

impl Files {
    pub fn new() -> Files {
        Files { files: Vec::new() }
    }

    pub fn add(&mut self, name: impl Into<String>, contents: impl Into<String>) -> FileId {
        let file_id = FileId(self.files.len());
        self.files.push(File {
            id: file_id,
            name: name.into(),
            contents: contents.into(),
        });
        file_id
    }

    pub fn byte_index(
        &self,
        file_id: FileId,
        line: impl Into<LineIndex>,
        column: impl Into<ColumnIndex>,
    ) -> Option<ByteIndex> {
        let source = &self[file_id].contents;
        let line = line.into();
        let column = column.into();
        let mut seen_lines = 0;
        let mut seen_bytes = 0;

        for (pos, _) in source.match_indices('\n') {
            if seen_lines == line.to_usize() {
                // FIXME: Column != byte width for larger unicode characters
                return Some(ByteIndex::from(seen_bytes + column.to_usize()));
            } else {
                seen_lines += 1;
                seen_bytes = pos + 1;
            }
        }

        None
    }

    pub fn location(&self, file_id: FileId, byte: impl Into<ByteIndex>) -> Option<Location> {
        let source = &self[file_id].contents;
        let byte = byte.into();
        let mut seen_lines = 0;
        let mut seen_bytes = 0;

        for (pos, _) in source.match_indices('\n') {
            if pos > byte.to_usize() {
                return Some(Location {
                    byte,
                    line: LineIndex::from(seen_lines),
                    // FIXME: Column != byte width for larger unicode characters
                    column: ColumnIndex::from(byte.to_usize() - seen_bytes),
                });
            } else {
                seen_lines += 1;
                seen_bytes = pos;
            }
        }

        None
    }

    pub fn line_span(&self, file_id: FileId, line: impl Into<LineIndex>) -> Option<Span<FileId>> {
        let source = &self[file_id].contents;
        let line = line.into();
        let mut seen_lines = 0;
        let mut seen_bytes = 0;

        for (pos, _) in source.match_indices('\n') {
            if seen_lines >= line.to_usize() {
                return Some(Span::new(file_id, seen_bytes, pos));
            } else {
                seen_lines += 1;
                seen_bytes = pos + 1;
            }
        }

        None
    }

    pub fn source(&self, span: Span<FileId>) -> Option<&str> {
        let start = span.start().to_usize();
        let end = span.end().to_usize();

        Some(&self[span.source()].contents[start..end])
    }
}

impl language_reporting::ReportingFiles for Files {
    type Span = Span<FileId>;
    type FileId = FileId;

    fn file_id(&self, span: Span<FileId>) -> FileId {
        span.source()
    }

    fn file_name(&self, file_id: FileId) -> language_reporting::FileName {
        language_reporting::FileName::Verbatim(self[file_id].name.clone())
    }

    fn byte_span(
        &self,
        file_id: FileId,
        from_index: usize,
        to_index: usize,
    ) -> Option<Span<FileId>> {
        let file_span = self[file_id].span();
        let span = Span::new(file_id, from_index, to_index);

        if file_span.contains(span) {
            Some(span)
        } else {
            None
        }
    }

    fn byte_index(&self, file_id: FileId, line: usize, column: usize) -> Option<usize> {
        Files::byte_index(self, file_id, line, column).map(ByteIndex::to_usize)
    }

    fn location(&self, file_id: FileId, index: usize) -> Option<language_reporting::Location> {
        Files::location(self, file_id, index).map(|location| language_reporting::Location {
            line: location.line.to_usize(),
            column: location.column.to_usize(),
        })
    }

    fn line_span(&self, file_id: FileId, line: usize) -> Option<Span<FileId>> {
        Files::line_span(self, file_id, line)
    }

    fn source(&self, span: Span<FileId>) -> Option<String> {
        Files::source(self, span).map(str::to_owned)
    }
}

impl ops::Index<FileId> for Files {
    type Output = File;

    fn index(&self, index: FileId) -> &File {
        &self.files[index.to_usize()]
    }
}