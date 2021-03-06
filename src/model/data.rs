///! Implements data structures for https://www.w3.org/TR/rdf11-concepts/
///! Inspired by [RDFjs](http://rdf.js.org/)
use std::fmt;
use std::option::Option;
use std::sync::Arc;
use std::sync::Mutex;

/// A RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri)
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct NamedNode {
    iri: String,
}

impl NamedNode {
    pub fn value(&self) -> &str {
        &self.iri
    }
}

impl fmt::Display for NamedNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}>", self.value())
    }
}

/// A RDF [blank node](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node)
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct BlankNode {
    id: String,
}

impl BlankNode {
    pub fn value(&self) -> &str {
        &self.id
    }
}

impl fmt::Display for BlankNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "_:{}", self.value())
    }
}

/// A RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal)
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Literal {
    SimpleLiteral(String),
    LanguageTaggedString { value: String, language: String },
    TypedLiteral { value: String, datatype: NamedNode },
}

lazy_static! {
    static ref XSD_STRING: NamedNode = NamedNode {
        iri: "http://www.w3.org/2001/XMLSchema#string".to_owned()
    };
    static ref RDF_LANG_STRING: NamedNode = NamedNode {
        iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".to_owned()
    };
}

impl Literal {
    /// The literal [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form)
    pub fn value(&self) -> &str {
        match self {
            Literal::SimpleLiteral(value) => value,
            Literal::LanguageTaggedString { value, .. } => value,
            Literal::TypedLiteral { value, .. } => value,
        }
    }

    /// The literal [language tag](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tag) if it is a [language-tagged string](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tagged-string)
    pub fn language(&self) -> Option<&str> {
        match self {
            Literal::LanguageTaggedString { language, .. } => Some(language),
            _ => None,
        }
    }

    /// The literal [datatype](https://www.w3.org/TR/rdf11-concepts/#dfn-datatype-iri)
    /// The datatype of [language-tagged string](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tagged-string) is always http://www.w3.org/1999/02/22-rdf-syntax-ns#langString
    pub fn datatype(&self) -> &NamedNode {
        match self {
            Literal::SimpleLiteral(_) => &XSD_STRING,
            Literal::LanguageTaggedString { .. } => &RDF_LANG_STRING,
            Literal::TypedLiteral { datatype, .. } => datatype,
        }
    }

    pub fn is_plain(&self) -> bool {
        match self {
            Literal::SimpleLiteral(_) => true,
            Literal::LanguageTaggedString { .. } => true,
            _ => false,
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_plain() {
            self.language()
                .map(|lang| write!(f, "\"{}\"@{}", self.value(), lang))
                .unwrap_or_else(|| write!(f, "\"{}\"", self.value()))
        } else {
            write!(f, "\"{}\"^^{}", self.value(), self.datatype())
        }
    }
}

/// The union of [IRIs](https://www.w3.org/TR/rdf11-concepts/#dfn-iri) and [blank nodes](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node).
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum NamedOrBlankNode {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
}

impl NamedOrBlankNode {
    pub fn value(&self) -> &str {
        match self {
            NamedOrBlankNode::NamedNode(node) => node.value(),
            NamedOrBlankNode::BlankNode(node) => node.value(),
        }
    }
}

impl fmt::Display for NamedOrBlankNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NamedOrBlankNode::NamedNode(node) => node.fmt(f),
            NamedOrBlankNode::BlankNode(node) => node.fmt(f),
        }
    }
}

impl From<NamedNode> for NamedOrBlankNode {
    fn from(node: NamedNode) -> Self {
        NamedOrBlankNode::NamedNode(node)
    }
}

impl From<BlankNode> for NamedOrBlankNode {
    fn from(node: BlankNode) -> Self {
        NamedOrBlankNode::BlankNode(node)
    }
}

/// A RDF [term](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-term)
/// It is the union of [IRIs](https://www.w3.org/TR/rdf11-concepts/#dfn-iri), [blank nodes](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node) and [literals](https://www.w3.org/TR/rdf11-concepts/#dfn-literal).
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Term {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
}

impl Term {
    pub fn value(&self) -> &str {
        match self {
            Term::NamedNode(node) => node.value(),
            Term::BlankNode(node) => node.value(),
            Term::Literal(literal) => literal.value(),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::NamedNode(node) => node.fmt(f),
            Term::BlankNode(node) => node.fmt(f),
            Term::Literal(literal) => literal.fmt(f),
        }
    }
}

impl From<NamedNode> for Term {
    fn from(node: NamedNode) -> Self {
        Term::NamedNode(node)
    }
}

impl From<BlankNode> for Term {
    fn from(node: BlankNode) -> Self {
        Term::BlankNode(node)
    }
}

impl From<Literal> for Term {
    fn from(literal: Literal) -> Self {
        Term::Literal(literal)
    }
}

impl From<NamedOrBlankNode> for Term {
    fn from(resource: NamedOrBlankNode) -> Self {
        match resource {
            NamedOrBlankNode::NamedNode(node) => Term::NamedNode(node),
            NamedOrBlankNode::BlankNode(node) => Term::BlankNode(node),
        }
    }
}

/// The interface of containers that looks like [RDF triples](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple)
pub trait TripleLike {
    /// The [subject](https://www.w3.org/TR/rdf11-concepts/#dfn-subject) of this triple
    fn subject(&self) -> &NamedOrBlankNode;

    /// The [subject](https://www.w3.org/TR/rdf11-concepts/#dfn-subject) of this triple
    fn subject_owned(self) -> NamedOrBlankNode;

    /// The [predicate](https://www.w3.org/TR/rdf11-concepts/#dfn-predicate) of this triple
    fn predicate(&self) -> &NamedNode;
    /// The [predicate](https://www.w3.org/TR/rdf11-concepts/#dfn-predicate) of this triple

    fn predicate_owned(self) -> NamedNode;

    /// The [object](https://www.w3.org/TR/rdf11-concepts/#dfn-object) of this triple
    fn object(&self) -> &Term;

    /// The [object](https://www.w3.org/TR/rdf11-concepts/#dfn-object) of this triple
    fn object_owned(self) -> Term;
}

/// A [RDF triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple)
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Triple {
    subject: NamedOrBlankNode,
    predicate: NamedNode,
    object: Term,
}

impl fmt::Display for Triple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} .", self.subject, self.predicate, self.object)
    }
}

impl TripleLike for Triple {
    fn subject(&self) -> &NamedOrBlankNode {
        return &self.subject;
    }

    fn subject_owned(self) -> NamedOrBlankNode {
        return self.subject;
    }

    fn predicate(&self) -> &NamedNode {
        return &self.predicate;
    }

    fn predicate_owned(self) -> NamedNode {
        return self.predicate;
    }

    fn object(&self) -> &Term {
        return &self.object;
    }

    fn object_owned(self) -> Term {
        return self.object;
    }
}

/// The interface of [triples](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple) that are in a [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset)
pub trait QuadLike: TripleLike {
    /// The name of the RDF [graph](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-graph) in which the triple is or None if it is in the [default graph](https://www.w3.org/TR/rdf11-concepts/#dfn-default-graph)
    fn graph_name(&self) -> &Option<NamedOrBlankNode>;

    /// The name of the RDF [graph](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-graph) in which the triple is or None if it is in the [default graph](https://www.w3.org/TR/rdf11-concepts/#dfn-default-graph)
    fn graph_name_owned(self) -> Option<NamedOrBlankNode>;
}

/// A [triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple) in a [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset)
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Quad {
    subject: NamedOrBlankNode,
    predicate: NamedNode,
    object: Term,
    graph_name: Option<NamedOrBlankNode>,
}

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.graph_name {
            Some(ref graph_name) => write!(
                f,
                "{} {} {} {} .",
                self.subject, self.predicate, self.object, graph_name
            ),
            None => write!(f, "{} {} {} .", self.subject, self.predicate, self.object),
        }
    }
}

impl TripleLike for Quad {
    fn subject(&self) -> &NamedOrBlankNode {
        return &self.subject;
    }

    fn subject_owned(self) -> NamedOrBlankNode {
        return self.subject;
    }

    fn predicate(&self) -> &NamedNode {
        return &self.predicate;
    }

    fn predicate_owned(self) -> NamedNode {
        return self.predicate;
    }

    fn object(&self) -> &Term {
        return &self.object;
    }

    fn object_owned(self) -> Term {
        return self.object;
    }
}

impl QuadLike for Quad {
    fn graph_name(&self) -> &Option<NamedOrBlankNode> {
        return &self.graph_name;
    }

    fn graph_name_owned(self) -> Option<NamedOrBlankNode> {
        return self.graph_name;
    }
}

/// An utility structure to generate bank node ids in a thread safe way
#[derive(Debug, Clone)]
struct U64IDProvider {
    counter: Arc<Mutex<u64>>,
}

impl U64IDProvider {
    pub fn next(&self) -> u64 {
        let mut id = self.counter.lock().unwrap();
        *id += 1;
        *id
    }
}

impl Default for U64IDProvider {
    fn default() -> Self {
        U64IDProvider {
            counter: Arc::new(Mutex::new(0)),
        }
    }
}

/// A structure creating RDF elements
#[derive(Debug, Clone)]
pub struct DataFactory {
    blank_node_id_provider: U64IDProvider,
}

impl Default for DataFactory {
    fn default() -> Self {
        DataFactory {
            blank_node_id_provider: U64IDProvider::default(),
        }
    }
}

impl DataFactory {
    /// Builds a RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri)
    pub fn named_node(&self, iri: impl Into<String>) -> NamedNode {
        NamedNode { iri: iri.into() }
    }

    /// Builds a RDF [blank node](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node) with a known id
    pub fn blank_node(&self, id: impl Into<String>) -> BlankNode {
        BlankNode { id: id.into() }
    }

    /// Builds a new RDF [blank node](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node) with a unique id
    pub fn new_blank_node(&self) -> BlankNode {
        self.blank_node(self.blank_node_id_provider.next().to_string())
    }

    /// Builds a RDF [simple literal](https://www.w3.org/TR/rdf11-concepts/#dfn-simple-literal)
    pub fn simple_literal(&self, value: impl Into<String>) -> Literal {
        Literal::SimpleLiteral(value.into())
    }

    /// Builds a RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal) with a [datatype](https://www.w3.org/TR/rdf11-concepts/#dfn-datatype-iri)
    pub fn typed_literal(
        &self,
        value: impl Into<String>,
        datatype: impl Into<NamedNode>,
    ) -> Literal {
        //TODO: find the best representation
        Literal::TypedLiteral {
            value: value.into(),
            datatype: datatype.into(),
        }
    }

    /// Builds a RDF [language-tagged string](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tagged-string)
    pub fn language_tagged_literal(
        &self,
        value: impl Into<String>,
        language: impl Into<String>,
    ) -> Literal {
        Literal::LanguageTaggedString {
            value: value.into(),
            language: language.into(),
        }
    }

    /// Builds a RDF [triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple)
    pub fn triple(
        &self,
        subject: impl Into<NamedOrBlankNode>,
        predicate: impl Into<NamedNode>,
        object: impl Into<Term>,
    ) -> Triple {
        Triple {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
        }
    }

    /// Builds a RDF [triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple) in a [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset)
    pub fn quad(
        &self,
        subject: impl Into<NamedOrBlankNode>,
        predicate: impl Into<NamedNode>,
        object: impl Into<Term>,
        graph_name: impl Into<Option<NamedOrBlankNode>>,
    ) -> Quad {
        Quad {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            graph_name: graph_name.into(),
        }
    }
}
