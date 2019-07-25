use crate::types::{
    StreamClaimOptions, StreamClaimReply, StreamInfoConsumersReply, StreamInfoGroupsReply,
    StreamInfoStreamReply, StreamMaxlen, StreamPendingCountReply, StreamPendingReply,
    StreamRangeReply, StreamReadOptions, StreamReadReply,
};

use redis::{cmd, ConnectionLike, FromRedisValue, RedisResult, ToRedisArgs};

/// Implementation of all redis stream commands.
///
pub trait StreamCommands: ConnectionLike + Sized {
    // XACK <key> <group> <id> <id> ... <id>

    /// Ack pending stream messages checked out by a consumer.
    ///
    #[inline]
    fn xack<K: ToRedisArgs, G: ToRedisArgs, ID: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        group: G,
        ids: &[ID],
    ) -> RedisResult<RV> {
        cmd("XACK").arg(key).arg(group).arg(ids).query(self)
    }

    // XADD key <ID or *> [field value] [field value] ...

    /// Add a stream message by `key`. Use `*` as the `id` for the current timestamp.
    ///
    #[inline]
    fn xadd<K: ToRedisArgs, ID: ToRedisArgs, F: ToRedisArgs, V: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        id: ID,
        items: &[(F, V)],
    ) -> RedisResult<RV> {
        cmd("XADD").arg(key).arg(id).arg(items).query(self)
    }

    // XADD key <ID or *> [rust BTreeMap] ...

    /// BTreeMap variant for adding a stream message by `key`.
    /// Use `*` as the `id` for the current timestamp.
    ///
    #[inline]
    fn xadd_map<K: ToRedisArgs, ID: ToRedisArgs, BTM: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        id: ID,
        map: BTM,
    ) -> RedisResult<RV> {
        cmd("XADD").arg(key).arg(id).arg(map).query(self)
    }

    // XADD key [MAXLEN [~|=] <count>] <ID or *> [field value] [field value] ...

    /// Add a stream message while capping the stream at a maxlength.
    ///
    #[inline]
    fn xadd_maxlen<
        K: ToRedisArgs,
        ID: ToRedisArgs,
        F: ToRedisArgs,
        V: ToRedisArgs,
        RV: FromRedisValue,
    >(
        &mut self,
        key: K,
        maxlen: StreamMaxlen,
        id: ID,
        items: &[(F, V)],
    ) -> RedisResult<RV> {
        cmd("XADD")
            .arg(key)
            .arg(maxlen)
            .arg(id)
            .arg(items)
            .query(self)
    }

    // XADD key [MAXLEN [~|=] <count>] <ID or *> [rust BTreeMap] ...

    /// BTreeMap variant for adding a stream message while capping the stream at a maxlength.
    ///
    #[inline]
    fn xadd_maxlen_map<K: ToRedisArgs, ID: ToRedisArgs, BTM: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        maxlen: StreamMaxlen,
        id: ID,
        map: BTM,
    ) -> RedisResult<RV> {
        cmd("XADD")
            .arg(key)
            .arg(maxlen)
            .arg(id)
            .arg(map)
            .query(self)
    }

    // XCLAIM <key> <group> <consumer> <min-idle-time> [<ID-1> <ID-2>]

    /// Claim pending, unacked messages, after some period of time,
    /// currently checked out by another consumer.
    ///
    /// This method only accepts the must-have arguments for claiming messages.
    /// If optional arguments are required, see `xclaim_options` below.
    ///
    #[inline]
    fn xclaim<K: ToRedisArgs, G: ToRedisArgs, C: ToRedisArgs, MIT: ToRedisArgs, ID: ToRedisArgs>(
        &mut self,
        key: K,
        group: G,
        consumer: C,
        min_idle_time: MIT,
        ids: &[ID],
    ) -> RedisResult<StreamClaimReply> {
        cmd("XCLAIM")
            .arg(key)
            .arg(group)
            .arg(consumer)
            .arg(min_idle_time)
            .arg(ids)
            .query(self)
    }

    // XCLAIM <key> <group> <consumer> <min-idle-time> <ID-1> <ID-2>
    //     [IDLE <milliseconds>] [TIME <mstime>] [RETRYCOUNT <count>]
    //     [FORCE] [JUSTID]

    /// This is the optional arguments version for claiming unacked, pending messages
    /// currently checked out by another consumer.
    ///
    /// ```no_run
    /// use redis_streams::{client_open,Connection,RedisResult,StreamCommands,StreamClaimOptions,StreamClaimReply};
    /// let client = client_open("redis://127.0.0.1/0").unwrap();
    /// let mut con = client.get_connection().unwrap();
    ///
    /// // Claim all pending messages for key "k1",
    /// // from group "g1", checked out by consumer "c1"
    /// // for 10ms with RETRYCOUNT 2 and FORCE
    ///
    /// let opts = StreamClaimOptions::default()
    ///     .with_force()
    ///     .retry(2);
    /// let results: RedisResult<StreamClaimReply> =
    ///     con.xclaim_options("k1", "g1", "c1", 10, &["0"], opts);
    ///
    /// // All optional arguments return a `Result<StreamClaimReply>` with one exception:
    /// // Passing JUSTID returns only the message `id` and omits the HashMap for each message.
    ///
    /// let opts = StreamClaimOptions::default()
    ///     .with_justid();
    /// let results: RedisResult<Vec<String>> =
    ///     con.xclaim_options("k1", "g1", "c1", 10, &["0"], opts);
    /// ```
    ///
    #[inline]
    fn xclaim_options<
        K: ToRedisArgs,
        G: ToRedisArgs,
        C: ToRedisArgs,
        MIT: ToRedisArgs,
        ID: ToRedisArgs,
        RV: FromRedisValue,
    >(
        &mut self,
        key: K,
        group: G,
        consumer: C,
        min_idle_time: MIT,
        ids: &[ID],
        options: StreamClaimOptions,
    ) -> RedisResult<RV> {
        cmd("XCLAIM")
            .arg(key)
            .arg(group)
            .arg(consumer)
            .arg(min_idle_time)
            .arg(ids)
            .arg(options)
            .query(self)
    }

    // XDEL <key> [<ID1> <ID2> ... <IDN>]

    /// Deletes a list of `id`s for a given stream `key`.
    ///
    #[inline]
    fn xdel<K: ToRedisArgs, ID: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        ids: &[ID],
    ) -> RedisResult<RV> {
        cmd("XDEL").arg(key).arg(ids).query(self)
    }

    // XGROUP CREATE <key> <groupname> <id or $>

    /// This command is used for creating a consumer `group`. It expects the stream key
    /// to already exist. Otherwise, use `xgroup_create_mkstream` if it doesn't.
    /// The `id` is the starting message id all consumers should read from. Use `$` If you want
    /// all consumers to read from the last message added to stream.
    ///
    #[inline]
    fn xgroup_create<K: ToRedisArgs, G: ToRedisArgs, ID: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        group: G,
        id: ID,
    ) -> RedisResult<RV> {
        cmd("XGROUP")
            .arg("CREATE")
            .arg(key)
            .arg(group)
            .arg(id)
            .query(self)
    }

    // XGROUP CREATE <key> <groupname> <id or $> [MKSTREAM]

    /// This is the alternate version for creating a consumer `group`
    /// which makes the stream if it doesn't exist.
    ///
    #[inline]
    fn xgroup_create_mkstream<
        K: ToRedisArgs,
        G: ToRedisArgs,
        ID: ToRedisArgs,
        RV: FromRedisValue,
    >(
        &mut self,
        key: K,
        group: G,
        id: ID,
    ) -> RedisResult<RV> {
        cmd("XGROUP")
            .arg("CREATE")
            .arg(key)
            .arg(group)
            .arg(id)
            .arg("MKSTREAM")
            .query(self)
    }

    // XGROUP SETID <key> <groupname> <id or $>

    /// Alter which `id` you want consumers to begin reading from an existing
    /// consumer `group`.
    ///
    #[inline]
    fn xgroup_setid<K: ToRedisArgs, G: ToRedisArgs, ID: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        group: G,
        id: ID,
    ) -> RedisResult<RV> {
        cmd("XGROUP")
            .arg("SETID")
            .arg(key)
            .arg(group)
            .arg(id)
            .query(self)
    }

    // XGROUP DESTROY <key> <groupname>

    /// Destroy an existing consumer `group` for a given stream `key`
    ///
    #[inline]
    fn xgroup_destroy<K: ToRedisArgs, G: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        group: G,
    ) -> RedisResult<RV> {
        cmd("XGROUP").arg("DESTROY").arg(key).arg(group).query(self)
    }

    // XGROUP DELCONSUMER <key> <groupname> <consumername>

    /// This deletes a `consumer` from an existing consumer `group`
    /// for given stream `key.
    ///
    #[inline]
    fn xgroup_delconsumer<K: ToRedisArgs, G: ToRedisArgs, C: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        group: G,
        consumer: C,
    ) -> RedisResult<RV> {
        cmd("XGROUP")
            .arg("DELCONSUMER")
            .arg(key)
            .arg(group)
            .arg(consumer)
            .query(self)
    }

    // XINFO CONSUMERS <key> <group>

    /// This returns all info details about
    /// which consumers have read messages for given consumer `group`.
    /// Take note of the StreamInfoConsumersReply return type.
    ///
    /// *It's possible this return value might not contain new fields
    /// added by Redis in future versions.*
    ///
    #[inline]
    fn xinfo_consumers<K: ToRedisArgs, G: ToRedisArgs>(
        &mut self,
        key: K,
        group: G,
    ) -> RedisResult<StreamInfoConsumersReply> {
        cmd("XINFO")
            .arg("CONSUMERS")
            .arg(key)
            .arg(group)
            .query(self)
    }

    // XINFO GROUPS <key>

    /// Returns all consumer `group`s created for a given stream `key`.
    /// Take note of the StreamInfoGroupsReply return type.
    ///
    /// *It's possible this return value might not contain new fields
    /// added by Redis in future versions.*
    ///
    #[inline]
    fn xinfo_groups<K: ToRedisArgs>(&mut self, key: K) -> RedisResult<StreamInfoGroupsReply> {
        cmd("XINFO").arg("GROUPS").arg(key).query(self)
    }

    // XINFO STREAM <key>

    /// Returns info about high-level stream details
    /// (first & last message `id`, length, number of groups, etc.)
    /// Take note of the StreamInfoStreamReply return type.
    ///
    /// *It's possible this return value might not contain new fields
    /// added by Redis in future versions.*
    ///
    #[inline]
    fn xinfo_stream<K: ToRedisArgs>(&mut self, key: K) -> RedisResult<StreamInfoStreamReply> {
        cmd("XINFO").arg("STREAM").arg(key).query(self)
    }

    // XLEN <key>
    /// Returns the number of messages for a given stream `key`.
    ///
    #[inline]
    fn xlen<K: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K) -> RedisResult<RV> {
        cmd("XLEN").arg(key).query(self)
    }

    // XPENDING <key> <group> [<start> <stop> <count> [<consumer>]]

    /// This is a basic version of making XPENDING command calls which only
    /// passes a stream `key` and consumer `group` and it
    /// returns details about which consumers have pending messages
    /// that haven't been acked.
    ///
    /// You can use this method along with
    /// `xclaim` or `xclaim_options` for determining which messages
    /// need to be retried.
    ///
    /// Take note of the StreamPendingReply return type.
    ///
    #[inline]
    fn xpending<K: ToRedisArgs, G: ToRedisArgs>(
        &mut self,
        key: K,
        group: G,
    ) -> RedisResult<StreamPendingReply> {
        cmd("XPENDING").arg(key).arg(group).query(self)
    }

    // XPENDING <key> <group> <start> <stop> <count>

    /// This XPENDING version returns a list of all messages over the range.
    /// You can use this for paginating pending messages (but without the message HashMap).
    ///
    /// Start and end follow the same rules `xrange` args. Set start to `-`
    /// and end to `+` for the entire stream.
    ///
    /// Take note of the StreamPendingCountReply return type.
    ///
    #[inline]
    fn xpending_count<
        K: ToRedisArgs,
        G: ToRedisArgs,
        S: ToRedisArgs,
        E: ToRedisArgs,
        C: ToRedisArgs,
    >(
        &mut self,
        key: K,
        group: G,
        start: S,
        end: E,
        count: C,
    ) -> RedisResult<StreamPendingCountReply> {
        cmd("XPENDING")
            .arg(key)
            .arg(group)
            .arg(start)
            .arg(end)
            .arg(count)
            .query(self)
    }

    // XPENDING <key> <group> <start> <stop> <count> <consumer>

    /// An alternate version of `xpending_count` which filters by `consumer` name.
    ///
    /// Start and end follow the same rules `xrange` args. Set start to `-`
    /// and end to `+` for the entire stream.
    ///
    /// Take note of the StreamPendingCountReply return type.
    ///
    #[inline]
    fn xpending_consumer_count<
        K: ToRedisArgs,
        G: ToRedisArgs,
        S: ToRedisArgs,
        E: ToRedisArgs,
        C: ToRedisArgs,
        CN: ToRedisArgs,
    >(
        &mut self,
        key: K,
        group: G,
        start: S,
        end: E,
        count: C,
        consumer: CN,
    ) -> RedisResult<StreamPendingCountReply> {
        cmd("XPENDING")
            .arg(key)
            .arg(group)
            .arg(start)
            .arg(end)
            .arg(count)
            .arg(consumer)
            .query(self)
    }

    // XRANGE key start end

    /// Returns a range of messages in a given stream `key`.
    ///
    /// Set `start` to `-` to begin at the first message.
    /// Set `end` to `+` to end the most recent message.
    /// You can pass message `id` to both `start` and `end`.
    ///
    /// Take note of the StreamRangeReply return type.
    ///
    #[inline]
    fn xrange<K: ToRedisArgs, S: ToRedisArgs, E: ToRedisArgs>(
        &mut self,
        key: K,
        start: S,
        end: E,
    ) -> RedisResult<StreamRangeReply> {
        cmd("XRANGE").arg(key).arg(start).arg(end).query(self)
    }

    // XRANGE key - +

    /// A helper method for automatically returning all messages in a stream by `key`.
    /// **Use with caution!**
    ///
    #[inline]
    fn xrange_all<K: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K) -> RedisResult<RV> {
        cmd("XRANGE").arg(key).arg("-").arg("+").query(self)
    }

    // XRANGE key start end [COUNT <n>]

    /// A method for paginating a stream by `key`.
    ///
    #[inline]
    fn xrange_count<K: ToRedisArgs, S: ToRedisArgs, E: ToRedisArgs, C: ToRedisArgs>(
        &mut self,
        key: K,
        start: S,
        end: E,
        count: C,
    ) -> RedisResult<StreamRangeReply> {
        cmd("XRANGE")
            .arg(key)
            .arg(start)
            .arg(end)
            .arg("COUNT")
            .arg(count)
            .query(self)
    }

    // XREAD STREAMS key_1 key_2 ... key_N ID_1 ID_2 ... ID_N

    /// Read a list of `id`s for each stream `key`.
    /// This is the basic form of reading streams.
    /// For more advanced control, like blocking, limiting, or reading by consumer `group`,
    /// see `xread_options`.
    ///
    #[inline]
    fn xread<K: ToRedisArgs, ID: ToRedisArgs>(
        &mut self,
        keys: &[K],
        ids: &[ID],
    ) -> RedisResult<StreamReadReply> {
        cmd("XREAD").arg("STREAMS").arg(keys).arg(ids).query(self)
    }

    // XREAD [BLOCK <milliseconds>] [COUNT <count>]
    //       STREAMS key_1 key_2 ... key_N
    //       ID_1 ID_2 ... ID_N
    // XREADGROUP [BLOCK <milliseconds>] [COUNT <count>] [NOACK] [GROUP group-name consumer-name]
    //       STREAMS key_1 key_2 ... key_N
    //       ID_1 ID_2 ... ID_N

    /// This method handles setting optional arguments for
    /// `XREAD` or `XREADGROUP` Redis commands.
    /// ```no_run
    /// use redis_streams::{client_open,Connection,RedisResult,StreamCommands,StreamReadOptions,StreamReadReply};
    /// let client = client_open("redis://127.0.0.1/0").unwrap();
    /// let mut con = client.get_connection().unwrap();
    ///
    /// // Read 10 messages from the start of the stream,
    /// // without registering as a consumer group.
    ///
    /// let opts = StreamReadOptions::default()
    ///     .count(10);
    /// let results: RedisResult<StreamReadReply> =
    ///     con.xread_options(&["k1"], &["0"], opts);
    ///
    /// // Read all undelivered messages for a given
    /// // consumer group. Be advised: the consumer group must already
    /// // exist before making this call. Also note: we're passing
    /// // '>' as the id here, which means all undelivered messages.
    ///
    /// let opts = StreamReadOptions::default()
    ///     .group("group-1", "consumer-1");
    /// let results: RedisResult<StreamReadReply> =
    ///     con.xread_options(&["k1"], &[">"], opts);
    /// ```
    ///
    #[inline]
    fn xread_options<K: ToRedisArgs, ID: ToRedisArgs>(
        &mut self,
        keys: &[K],
        ids: &[ID],
        options: StreamReadOptions,
    ) -> RedisResult<StreamReadReply> {
        cmd(if options.read_only() {
            "XREAD"
        } else {
            "XREADGROUP"
        })
        .arg(options)
        .arg("STREAMS")
        .arg(keys)
        .arg(ids)
        .query(self)
    }

    // XREVRANGE key end start

    /// This is the reverse version of `xrange`.
    /// The same rules apply for `start` and `end` here.
    ///
    #[inline]
    fn xrevrange<K: ToRedisArgs, E: ToRedisArgs, S: ToRedisArgs>(
        &mut self,
        key: K,
        end: E,
        start: S,
    ) -> RedisResult<StreamRangeReply> {
        cmd("XREVRANGE").arg(key).arg(end).arg(start).query(self)
    }

    // XREVRANGE key + -

    /// This is the reverse version of `xrange_all`.
    /// The same rules apply for `start` and `end` here.
    ///
    fn xrevrange_all<K: ToRedisArgs>(&mut self, key: K) -> RedisResult<StreamRangeReply> {
        cmd("XREVRANGE").arg(key).arg("+").arg("-").query(self)
    }

    // XREVRANGE key end start [COUNT <n>]

    /// This is the reverse version of `xrange_count`.
    /// The same rules apply for `start` and `end` here.
    ///
    #[inline]
    fn xrevrange_count<K: ToRedisArgs, E: ToRedisArgs, S: ToRedisArgs, C: ToRedisArgs>(
        &mut self,
        key: K,
        end: E,
        start: S,
        count: C,
    ) -> RedisResult<StreamRangeReply> {
        cmd("XREVRANGE")
            .arg(key)
            .arg(end)
            .arg(start)
            .arg("COUNT")
            .arg(count)
            .query(self)
    }

    // XTRIM <key> MAXLEN [~|=] <count>  (Same as XADD MAXLEN option)

    /// Trim a stream `key` to a MAXLEN count.
    ///
    #[inline]
    fn xtrim<K: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        maxlen: StreamMaxlen,
    ) -> RedisResult<RV> {
        cmd("XTRIM").arg(key).arg(maxlen).query(self)
    }
}

impl<T> StreamCommands for T where T: ConnectionLike {}
