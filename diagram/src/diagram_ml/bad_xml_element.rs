//ti MLEvent for BadXMLElement
struct BadXMLElement {
}

//ii MLEvent for BadXMLElement
impl <'a, R:hml::Reader> MLEvent <'a, R, BadXMLElement> for BadXMLElement {
    fn ml_new(reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor, span:&Span<R>, tag:hml::Tag) -> Result<Self, MLError<R>> {
        let s = Self {};
        Self::ml_event(s, reader, descriptor)
    }
    fn ml_event (s:Self, reader:&mut MLReader<R>, descriptor:&DiagramDescriptor) -> Result<Self, MLError<R>> {
        match reader.next_event()? {
            MarkupEvent::StartElement{bounds, name, attributes, ..} => { // element in bad element - just consume
                let r = BadXMLElement::ml_new(reader, descriptor, &bounds, &name.name, &attributes);
                reader.errors.update(r);
            }
            MarkupEvent::EndElement{..}         => { return Ok(s); } // end the use
            MarkupEvent::Comment{..}            => (), // continue
            MarkupEvent::Content{..}            => (), // continue
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        Self::ml_event(s, reader, descriptor)
    }
}

