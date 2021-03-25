use platform::hal::mailbox::*;
struct WaitEvent;
impl MBRpc for WaitEvent {
    type REQ = u32;
    type RESP = u32;
    fn put_req(&self, req: Self::REQ, entry: &mut MBReqEntry) {
        entry.words = 1;
        entry.action = MBAction::OTHER;
        entry.args[0] = 0x8;
        entry.args[1] = req;
    }
    fn get_resp(&self, resp: &MBRespEntry) -> Self::RESP {
        resp.rets
    }
}

pub fn mb_wait_event(event: u32) -> u32 {
    (&MB_SENDER).send(&WaitEvent, event)
}
