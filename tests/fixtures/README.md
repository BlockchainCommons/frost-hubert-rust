# XID Fixtures

Test fixtures in this directory were generated with the `envelope` CLI, which is already available elsewhere in this workspace (`bc-envelope-cli`). Commands follow the documented flows in `bc-envelope-cli/docs/XID.md`:

```sh
# Alice: signed inception document without embedded private keys
read -r ALICE_PRV ALICE_PUB <<< "$(envelope generate keypairs)"
SIGNED_ALICE=$(envelope xid new "$ALICE_PRV" --private omit --nickname "Alice" --sign inception)
printf "%s" "$SIGNED_ALICE" > alice_signed_xid.txt

# Bob: signed and unsigned variants
read -r BOB_PRV BOB_PUB <<< "$(envelope generate keypairs)"
SIGNED_BOB=$(envelope xid new "$BOB_PRV" --private omit --nickname "Bob" --sign inception)
UNSIGNED_BOB=$(envelope xid new "$BOB_PUB" --private omit --nickname "Bob")
printf "%s" "$SIGNED_BOB" > bob_signed_xid.txt
printf "%s" "$UNSIGNED_BOB" > bob_unsigned_xid.txt

# Alice/Bob: unsigned documents that include private keys (for tests that need to sign)
ALICE_PRIVATE=$(envelope xid new "$ALICE_PRV" --private include --nickname "Alice" --sign none)
BOB_PRIVATE=$(envelope xid new "$BOB_PRV" --private include --nickname "Bob" --sign none)
printf "%s" "$ALICE_PRIVATE" > alice_private_xid.txt
printf "%s" "$BOB_PRIVATE" > bob_private_xid.txt

# Carol: signed and private-including variants
read -r CAROL_PRV CAROL_PUB <<< "$(envelope generate keypairs)"
SIGNED_CAROL=$(envelope xid new "$CAROL_PRV" --private omit --nickname "Carol" --sign inception)
PRIVATE_CAROL=$(envelope xid new "$CAROL_PRV" --private include --nickname "Carol" --sign none)
printf "%s" "$CAROL_PRV" > carol_prvkeys.txt
printf "%s" "$CAROL_PUB" > carol_pubkeys.txt
printf "%s" "$SIGNED_CAROL" > carol_signed_xid.txt
printf "%s" "$PRIVATE_CAROL" > carol_private_xid.txt

# Dan: signed and private-including variants
read -r DAN_PRV DAN_PUB <<< "$(envelope generate keypairs)"
SIGNED_DAN=$(envelope xid new "$DAN_PRV" --private omit --nickname "Dan" --sign inception)
PRIVATE_DAN=$(envelope xid new "$DAN_PRV" --private include --nickname "Dan" --sign none)
printf "%s" "$DAN_PRV" > dan_prvkeys.txt
printf "%s" "$DAN_PUB" > dan_pubkeys.txt
printf "%s" "$SIGNED_DAN" > dan_signed_xid.txt
printf "%s" "$PRIVATE_DAN" > dan_private_xid.txt
```

The accompanying `*_prvkeys` and `*_pubkeys` files capture the keypairs used to mint the fixtures and can be regenerated in the same manner if needed. All strings are stored without trailing newlines for easier comparison inside tests.
