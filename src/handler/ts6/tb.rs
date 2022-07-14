use chrono::NaiveDateTime;

use crate::handler::{Error, Outcome};
use crate::hostmask::Hostmask;
use crate::line::Line;
use crate::network::Network;
use crate::topic::Topic;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    //:420 TB #gaynet 1640815950 jess!~jess@husky.vpn.lolnerd.net :gay stuff itc
    pub fn handle_tb(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
        Error::assert_arg_count(line, 4..)?;

        let channel = network.get_channel_mut(&line.args[0])?;
        let since = line.args[1]
            .decode()
            .parse::<i64>()
            .map_err(|_| Error::BadArgument)?;

        let topic = Topic {
            text: line.args[3].decode(),
            since: NaiveDateTime::from_timestamp(since, 0),
            //TODO: handle missing setter
            setter: Hostmask::try_from(line.args[2].decode().as_str())
                .map_err(|_| Error::BadArgument)?,
        };
        channel.topic = Some(topic);

        Ok(Outcome::Empty)
    }
}
