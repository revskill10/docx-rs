use serde::Serialize;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct NumPages {
    pub instr: InstrNUMPAGES,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_property: Option<FrameProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_property: Option<ParagraphProperty>,
}

impl Default for NumPages {
    fn default() -> Self {
        Self {
            instr: InstrNUMPAGES {},
            frame_property: None,
            paragraph_property: None,
        }
    }
}

impl NumPages {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn wrap(mut self, wrap: impl Into<String>) -> Self {
        self.frame_property = Some(FrameProperty {
            wrap: Some(wrap.into()),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn v_anchor(mut self, anchor: impl Into<String>) -> Self {
        self.frame_property = Some(FrameProperty {
            v_anchor: Some(anchor.into()),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn h_anchor(mut self, anchor: impl Into<String>) -> Self {
        self.frame_property = Some(FrameProperty {
            h_anchor: Some(anchor.into()),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn h_rule(mut self, r: impl Into<String>) -> Self {
        self.frame_property = Some(FrameProperty {
            h_rule: Some(r.into()),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn x_align(mut self, align: impl Into<String>) -> Self {
        self.frame_property = Some(FrameProperty {
            x_align: Some(align.into()),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn y_align(mut self, align: impl Into<String>) -> Self {
        self.frame_property = Some(FrameProperty {
            y_align: Some(align.into()),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn h_space(mut self, x: i32) -> Self {
        self.frame_property = Some(FrameProperty {
            h_space: Some(x),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn v_space(mut self, x: i32) -> Self {
        self.frame_property = Some(FrameProperty {
            v_space: Some(x),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn x(mut self, x: i32) -> Self {
        self.frame_property = Some(FrameProperty {
            x: Some(x),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.frame_property = Some(FrameProperty {
            y: Some(y),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn width(mut self, n: u32) -> Self {
        self.frame_property = Some(FrameProperty {
            w: Some(n),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn height(mut self, n: u32) -> Self {
        self.frame_property = Some(FrameProperty {
            h: Some(n),
            ..self.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn align(mut self, alignment_type: AlignmentType) -> Self {
        self.paragraph_property = Some(
            self.paragraph_property
                .unwrap_or_default()
                .align(alignment_type),
        );
        self
    }

    fn inner_build(&self) -> Vec<u8> {
        let p = StructuredDataTagProperty::new();
        let mut b = XMLBuilder::new();

        b = b
            .open_structured_tag()
            .add_child(&p)
            .open_structured_tag_content();

        let mut p = Paragraph::new().add_run(
            Run::new()
                .add_field_char(FieldCharType::Begin, false)
                .add_instr_text(InstrText::NUMPAGES(self.instr.clone()))
                .add_field_char(FieldCharType::Separate, false)
                .add_text("1")
                .add_field_char(FieldCharType::End, false),
        );

        if let Some(ref pr) = self.paragraph_property {
            p.property = pr.clone();
        }

        if let Some(ref f) = self.frame_property {
            p.property.frame_property = Some(f.clone());
        }

        b = b.add_child(&p);
        b = b.close().close();
        b.build()
    }
}

impl BuildXML for NumPages {
    fn build(&self) -> Vec<u8> {
        self.inner_build()
    }
}

impl BuildXML for Box<NumPages> {
    fn build(&self) -> Vec<u8> {
        self.inner_build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_page() {
        let b = NumPages::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sdt><w:sdtPr><w:rPr /></w:sdtPr><w:sdtContent><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="begin" w:dirty="false" /><w:instrText>PAGE</w:instrText><w:fldChar w:fldCharType="separate" w:dirty="false" /><w:t xml:space="preserve">1</w:t><w:fldChar w:fldCharType="end" w:dirty="false" /></w:r></w:p></w:sdtContent>
</w:sdt>"#
        );
    }

    #[test]
    fn test_page_with_wrap() {
        let b = NumPages::new().wrap("none").x_align("left").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sdt><w:sdtPr><w:rPr /></w:sdtPr><w:sdtContent><w:p w14:paraId="12345678"><w:pPr><w:rPr /><w:framePr w:wrap="none" w:xAlign="left" /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="begin" w:dirty="false" /><w:instrText>PAGE</w:instrText><w:fldChar w:fldCharType="separate" w:dirty="false" /><w:t xml:space="preserve">1</w:t><w:fldChar w:fldCharType="end" w:dirty="false" /></w:r></w:p></w:sdtContent>
</w:sdt>"#
        );
    }
}
