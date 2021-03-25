SECTIONS
{
    .mailbox (NOLOAD) : {
        _smailbox = .;
        /* Place init sections first */
        KEEP(*(.mailbox_queue));
        _emailbox = .;
    } > REGION_MAILBOX
}