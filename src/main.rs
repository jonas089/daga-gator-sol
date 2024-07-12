pub mod storage;
pub mod types;
use colored::*;

fn main() {
    println!(
        r#"
           .-._   _ _ _ _ _ _ _ _
.-''-.__.-'00  '-' ' ' ' ' ' ' ' '-.
'.___ '    .   .--_'-' '-' '-' _'-' '._
 V: V 'vv-'   '_   '.       .'  _..' '.'.
   '=.____.=_.--'   :_.__.__:_   '.   : :
           (((____.-'        '-.  /   : :
                             (((-'\ .' /
                           _____..'  .'
                          '-._____.-'"#
    );
    println!(
        "{}{} {}!",
        "Data".green(),
        "Gator".yellow(),
        "says Hello".blue()
    );
}

// Devnotes - Plan of Attack
/*
When it boots up it starts collecting Blocks from the
current epoch onwards.

```
current_epoch = x
index = x.min
fetch_blocks(x.min, x.max)
    index ++

if index == x.max{
    query current_epoch every x seconds
    -> until a new epoch is found
}

Comment: Due to time-constraints I am not considering
an event listener. If an event is emitted by the
SSE once a new era starts, then this could be used
instead of a continuous query in a more mature applicatoin.
```
*/

/* Required API methods
getEpochInfo
getBlocks(start_slot, end_slot)
*/

/* Block queries
In-memory DB.
Query Blocks for current Epoch only.
e.g. slot-range.
Parse Blocks into a serializable struct with serde-json.
*/

/* API endpoints
    return information about Blocks
    and Transactions,- this will
    require adjustments to the
    Block parsing to also serialize
    each Transaction individually
    and insert it into the MemoryDB.
*/
