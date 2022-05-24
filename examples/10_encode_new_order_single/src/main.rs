use decimal::d128;
use fefix::prelude::*;
use fefix::tagvalue::{Config, Encoder};
use rust_decimal_macros::dec;

// 8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072|

macro_rules! fix {
    ($($field:tt)+) => { fix44::$($field)+ }
}

fn main() {
    let mut encoder = fix_encoder();
    let mut buffer = Vec::new();
    let mut msg = encoder.start_message(b"FIX.4.4", &mut buffer, b"ExecutionReport");

    msg.set(fix!(MSG_SEQ_NUM), 215);
    msg.set(fix!(SENDER_COMP_ID), "CLIENT12");
    msg.set(fix!(TARGET_COMP_ID), "B");
    msg.set(fix!(ACCOUNT), "Marcel");
    msg.set(fix!(CL_ORD_ID), "13346");
    msg.set(
        fix!(HANDL_INST),
        fix!(HandlInst::AutomatedExecutionOrderPrivateNoBrokerIntervention)
    );
    msg.set(fix!(ORD_TYPE), fix!(OrdType::Limit));
    msg.set(fix!(PRICE), dec!(150.08));
    msg.set(fix!(PRICE_DELTA), d128!(32.99));
    msg.set(fix!(SIDE), fix!(Side::Buy));
    msg.set(fix!(TIME_IN_FORCE), fix!(TimeInForce::Day));

    let ss = msg.done();
    let s = String::from_utf8(ss.0.to_vec()).expect("Found invalid UTF-8").replace("\u{1}", "|");
    println!("{:?}", s);
}

fn fix_encoder() -> Encoder<Config> {
    Encoder::default()
}
