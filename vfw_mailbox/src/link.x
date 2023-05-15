SECTIONS
{
    .mailbox (NOLOAD) : {
        . = ALIGN(8);
        _smailbox = .;
        KEEP(*(.mailbox_queue));
        _emailbox = .;
    } > REGION_MAILBOX
}