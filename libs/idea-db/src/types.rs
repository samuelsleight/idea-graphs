use crate::key::Key;

pub struct ThingKind {
    pub name: String
}

pub struct ConnectionKind {
    pub name: String,
    pub lhs: Key<ThingKind>,
    pub rhs: Key<ThingKind>
}

pub struct Thing {
    pub kind: Key<ThingKind>,
    pub name: String,
    pub x: i64,
    pub y: i64
}

pub struct Connection {
    pub kind: Key<ConnectionKind>,
    pub lhs: Key<Thing>,
    pub rhs: Key<Thing>,
}

pub struct FullThing {
    pub key: Key<Thing>,
    pub thing: Thing,
    pub kind: ThingKind
}

pub struct FullConnection {
    pub key: Key<Connection>,
    pub connection: Connection,
    pub kind: ConnectionKind,
    pub from: (i64, i64),
    pub to: (i64, i64)
}
