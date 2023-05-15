SECTIONS
{
    .mailbox (NOLOAD) : {
        _smailbox = .;
        KEEP(*(.mailbox_queue));
        _emailbox = .;
    } > REGION_MAILBOX
}