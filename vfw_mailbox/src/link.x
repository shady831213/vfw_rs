SECTIONS
{
    .mailbox : {
        _smailbox = .;
        KEEP(*(.mailbox_queue));
        _emailbox = .;
    } > REGION_MAILBOX
}