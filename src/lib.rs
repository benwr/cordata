use std::collections::BTreeMap;
use std::default::Default;

// UUIDs are way bigger than we need, and we do care about the size of
// this struct. If there were 10^9 nodes collaborating on the same
// document, 10 bytes give a ~10^-8  chance of a collision. With
// "only" 10^6 nodes collaborating, you get ~10^-13. And at that point,
// you'd need >10 megabytes to even store the vector clock.
// I'll take those odds.
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ThreadId([u8; 10]);
// TODO: tricky question is whether we should try to avoid (simple) construction of specific
// NodeIds. Not allowing it could protect people from shooting themselves in the foot, but 
// we have to be very careful with the rest of the API to ensure that this is never necessary.

// How large do the ints in our version vectors need to be? The allegedly longest novel in the
// world is In Search of Lost Time, at ~10M characters. So you'd need to do the work of 400
// Prousts, all on the same computer, to overflow a u32. #[derive(Debug, Eq, PartialEq,
// PartialOrd)] struct VersionVector<N, V>(BTreeMap<N, V>);

// Thus, for now, a (compact) VersionVector is 14 bytes * node count. My guess is that a typical
// document will have less than, I dunno, order 1k editor nodes? Even if individual sessions get
// their own nodes, which is usually avoidable using browser / desktop local storage. So the
// version vector will be roughly 20KiB. I'm okay with this, as long as there's only a single
// version vector. And I think we can enforce this at the node level.
struct VersionVector<T, V>(BTreeMap<T, V>);

pub struct State<Operation, OperationId, ThreadId, LocalClock> {
    thread_id: ThreadId,
    version: VersionVector<ThreadId, LocalClock>,
    operations: BTreeMap<OperationId, Operation>,
}


// things we'll need from the user:
//  - a way to get an OperationId from an Operation

impl<Operation, OperationId, ThreadId, LocalClock> State<Operation, OperationId, ThreadId, LocalClock> {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
