pub mod x11rb;

use std::collections::HashMap;

use xim_parser::{Attribute, Writer, XimWrite};

pub struct NestedListBuilder<'a> {
    id_map: &'a HashMap<String, u16>,
    out: &'a mut Vec<Attribute>,
}

impl<'a> NestedListBuilder<'a> {
    pub fn push<V: XimWrite>(self, name: &str, value: V) -> Self {
        if let Some(id) = self.id_map.get(name).copied() {
            let mut buf = Vec::with_capacity(value.size());
            value.write(&mut Writer::new(&mut buf));
            self.out.push(Attribute { id, value: buf });
        }

        self
    }
}

pub struct AttributeBuilder<'a> {
    id_map: &'a HashMap<String, u16>,
    out: Vec<Attribute>,
}

impl<'a> AttributeBuilder<'a> {
    pub fn push<V: XimWrite>(mut self, name: &str, value: V) -> Self {
        if let Some(id) = self.id_map.get(name).copied() {
            let mut buf = Vec::with_capacity(value.size());
            value.write(&mut Writer::new(&mut buf));
            self.out.push(Attribute { id, value: buf });
        }

        self
    }

    pub fn nested_list(mut self, name: &str, f: impl FnOnce(NestedListBuilder)) -> Self {
        if let Some(sep_id) = self.id_map.get("separatorofNestedList").copied() {
            if let Some(id) = self.id_map.get(name).copied() {
                self.out.push(Attribute {
                    id,
                    value: Vec::new(),
                });
                f(NestedListBuilder {
                    id_map: self.id_map,
                    out: &mut self.out,
                });
                self.out.push(Attribute {
                    id: sep_id,
                    value: Vec::new(),
                });
            }
        }

        self
    }

    pub fn build(self) -> Vec<Attribute> {
        self.out
    }
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug)]
struct Atoms<Atom> {
    XIM_SERVERS: Atom,
    LOCALES: Atom,
    TRANSPORT: Atom,
    XIM_XCONNECT: Atom,
    XIM_PROTOCOL: Atom,
    DATA: Atom,
}

impl<Atom> Atoms<Atom> {
    #[allow(unused)]
    pub fn new<E, F>(f: F) -> Result<Self, E>
    where
        F: Fn(&'static str) -> Result<Atom, E>,
    {
        Ok(Self {
            XIM_SERVERS: f("XIM_SERVERS")?,
            LOCALES: f("LOCALES")?,
            TRANSPORT: f("TRANSPORT")?,
            XIM_XCONNECT: f("_XIM_XCONNECT")?,
            XIM_PROTOCOL: f("_XIM_PROTOCOL")?,
            DATA: f("XIM_RS_DATA")?,
        })
    }
}
