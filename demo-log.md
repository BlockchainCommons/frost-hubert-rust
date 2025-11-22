## Set zsh options

zsh is the default shell on macOS and many Linux systems. This keeps history markers out of the transcript.

```
setopt nobanghist
```

## Checking prerequisites

Verify that the required CLI tools are present and available in $PATH.

```
for cmd in frost envelope; do
  $cmd --version
done

│ frost 0.1.0
│ bc-envelope-cli 0.27.0
```

## Preparing demo workspace

Start with a clean directory to capture demo artifacts.

```
rm -rf demo && mkdir -p demo
```

## Provisioning XID for Alice

Generate Alice's key material, a private XID document (for owner use), and a signed public XID document (for participants).

```
ALICE_PRVKEYS=$(envelope generate prvkeys)
echo "ALICE_PRVKEYS=$ALICE_PRVKEYS"
ALICE_PUBKEYS=$(envelope generate pubkeys "$ALICE_PRVKEYS")
echo "ALICE_PUBKEYS=$ALICE_PUBKEYS"
ALICE_OWNER_DOC=$(envelope xid new --nickname Alice --sign inception "$ALICE_PRVKEYS")
echo "ALICE_OWNER_DOC=$ALICE_OWNER_DOC"
ALICE_SIGNED_DOC=$(envelope xid new --nickname Alice --private omit --sign inception "$ALICE_PRVKEYS")
echo "ALICE_SIGNED_DOC=$ALICE_SIGNED_DOC"

│ ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxsefnoxfyutgrurytrtbgoxcfeooefsjtyasksftaksdtdkhdetayvlzcpmsemsehtansgehdcxskwpftqzihhlettlhevefpjslrptzepahfsnpegstivebymnbtsaqdztkosetamolusaaals
│ ALICE_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxcwoxjoidbafdwfjydposesrddinypthlclssvedsemknghvwchpkenskksmwtsjotansgrhdcxjzieihkkftaypmqzjnbdwmpklptbhyjzjlbnbdbzhhmocwmohhzehezotbdkfwdabgnnsofs
│ ALICE_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxfpfrluaymknymkoeutndsapfadpkfnaasggobgmoaocltbfnltynzeceyksflpetoyaylrtpsotansgylftanshfhdcxcwoxjoidbafdwfjydposesrddinypthlclssvedsemknghvwchpkenskksmwtsjotansgrhdcxjzieihkkftaypmqzjnbdwmpklptbhyjzjlbnbdbzhhmocwmohhzehezotbdkfwdaoycsfncsfgoycscstpsoihfpjziniaihlfoycsfptpsotansgtlftansgohdcxsefnoxfyutgrurytrtbgoxcfeooefsjtyasksftaksdtdkhdetayvlzcpmsemsehtansgehdcxskwpftqzihhlettlhevefpjslrptzepahfsnpegstivebymnbtsaqdztkosetamooybstpsotansgmhdcxlgnnsspahddkldgyuewmfdleswqdveinlefptsftwftkdnnnhgfwneecpkaddisboyaxtpsotansghhdfzhdstbtfegaclplgsiooldasfdsgabdjzfgwloxnshnytglaycycnfyctweaahybkdliohltymuhgspvesandgalpioldgrfndncmfxmtroctrtjtgoytswlnbnwpvyhhnegyoskg
│ ALICE_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxfpfrluaymknymkoeutndsapfadpkfnaasggobgmoaocltbfnltynzeceyksflpetoyaylstpsotansgylftanshfhdcxcwoxjoidbafdwfjydposesrddinypthlclssvedsemknghvwchpkenskksmwtsjotansgrhdcxjzieihkkftaypmqzjnbdwmpklptbhyjzjlbnbdbzhhmocwmohhzehezotbdkfwdaoycsfncsfgoycscstpsoihfpjziniaihoyaxtpsotansghhdfzstswwkfnhycfaecktbzehgwnssgypswewshsbgjnfnzecxhgvwzevtjkhtstdioedlsemdspimdkstvewyfzgddkiychjpmytoclldspiycxhdrtdmtnehgradcapkdyrfjevoax
```

## Provisioning XID for Bob

Generate Bob's key material, a private XID document (for owner use), and a signed public XID document (for participants).

```
BOB_PRVKEYS=$(envelope generate prvkeys)
echo "BOB_PRVKEYS=$BOB_PRVKEYS"
BOB_PUBKEYS=$(envelope generate pubkeys "$BOB_PRVKEYS")
echo "BOB_PUBKEYS=$BOB_PUBKEYS"
BOB_OWNER_DOC=$(envelope xid new --nickname Bob --sign inception "$BOB_PRVKEYS")
echo "BOB_OWNER_DOC=$BOB_OWNER_DOC"
BOB_SIGNED_DOC=$(envelope xid new --nickname Bob --private omit --sign inception "$BOB_PRVKEYS")
echo "BOB_SIGNED_DOC=$BOB_SIGNED_DOC"

│ BOB_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxurhphkdrbwcwrknebyondpryprdehtdplygyneosietpylgwnbfwwnaehddabdtptansgehdcxvthgswtpfyqzsakpjnkpswrelrnngwbgveindpvytlwsmhoerhnddkksvwecfloslkpkurfx
│ BOB_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxcahkhtfdgopakpbzvardwmcletldsbltzshkteehonvyksollohfkelomhskkssbtansgrhdcxparpkbjtspdretaavswdbghdhhfxkbkesawnflykidoxgsldbzaylfsaiewsrngowynyrons
│ BOB_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxhsktpyhseebasnenjkisaejnjohpplsackimwyktcmqzgdjnwfhlmnhtdshdbgwdoyaylrtpsotansgylftanshfhdcxcahkhtfdgopakpbzvardwmcletldsbltzshkteehonvyksollohfkelomhskkssbtansgrhdcxparpkbjtspdretaavswdbghdhhfxkbkesawnflykidoxgsldbzaylfsaiewsrngooycsfncsfglfoycsfptpsotansgtlftansgohdcxurhphkdrbwcwrknebyondpryprdehtdplygyneosietpylgwnbfwwnaehddabdtptansgehdcxvthgswtpfyqzsakpjnkpswrelrnngwbgveindpvytlwsmhoerhnddkksvwecflosoybstpsotansgmhdcxuewdbegujnwkcflnssgdhgtsmnclcpoycawprkadrylfbsoelffnttylpaaaahckoycscstpsoiafwjlidoyaxtpsotansghhdfzbtnnflcxmhynihamtnttprrfjphnkbztdsndjnfhimmsatswembysafgatahlotytlttcffgyttosesflaghmhbybthtjkbdfhwnbbvdurgutlhscenewsflrpbsfzattiidcltk
│ BOB_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxhsktpyhseebasnenjkisaejnjohpplsackimwyktcmqzgdjnwfhlmnhtdshdbgwdoyaylstpsotansgylftanshfhdcxcahkhtfdgopakpbzvardwmcletldsbltzshkteehonvyksollohfkelomhskkssbtansgrhdcxparpkbjtspdretaavswdbghdhhfxkbkesawnflykidoxgsldbzaylfsaiewsrngooycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzgwpdytrhhfjyoymotsykbthfuturjolgctpewfgtfzsnkthghkrskbzeurkgnbonsauecfjycnkbtantlpwztkgraohkgwlsluwzhfpafhrsdrsrynmejnvljndnchcplydeimnl
```

## Provisioning XID for Carol

Generate Carol's key material, a private XID document (for owner use), and a signed public XID document (for participants).

```
CAROL_PRVKEYS=$(envelope generate prvkeys)
echo "CAROL_PRVKEYS=$CAROL_PRVKEYS"
CAROL_PUBKEYS=$(envelope generate pubkeys "$CAROL_PRVKEYS")
echo "CAROL_PUBKEYS=$CAROL_PUBKEYS"
CAROL_OWNER_DOC=$(envelope xid new --nickname Carol --sign inception "$CAROL_PRVKEYS")
echo "CAROL_OWNER_DOC=$CAROL_OWNER_DOC"
CAROL_SIGNED_DOC=$(envelope xid new --nickname Carol --private omit --sign inception "$CAROL_PRVKEYS")
echo "CAROL_SIGNED_DOC=$CAROL_SIGNED_DOC"

│ CAROL_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxlsrsztrsrktbbkwygyfdfmjtjsssbtdidyonmwrkdakogucagdfgtnmkcmsapdehtansgehdcxlprtbzdspyvltoiofychckvegwghprlygatkaopftybniapdrspfbamyvyiefmweeywfcstd
│ CAROL_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxqzaavtmorloshpfmenimcefdclynlydedpfdztcfvsqzlbbbahjlfhltknmshfhktansgrhdcxcewevshesreclkbgfglyhkettprybbbwjelndpmohgtbjlrezmfeaycytewycmecktgugapy
│ CAROL_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxihksbbgdcsfypedrwpmdpkloeespbyflwlgtgtynhduyeclyvybtkewysbsnnymwoyaylrtpsotansgylftanshfhdcxqzaavtmorloshpfmenimcefdclynlydedpfdztcfvsqzlbbbahjlfhltknmshfhktansgrhdcxcewevshesreclkbgfglyhkettprybbbwjelndpmohgtbjlrezmfeaycytewycmecoycsfncsfglfoycsfptpsotansgtlftansgohdcxlsrsztrsrktbbkwygyfdfmjtjsssbtdidyonmwrkdakogucagdfgtnmkcmsapdehtansgehdcxlprtbzdspyvltoiofychckvegwghprlygatkaopftybniapdrspfbamyvyiefmweoybstpsotansgmhdcxlohtwehehtsgadbdfhnsmwwerdwldygosspkkpweaxsnnbmybwglrhglvlcxglhhoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzeefecfmhdibalybtytcsfhrefrjkfemhbyamckdnrhlomwdlwefdwleogyoxbadnkpotishscpesmtrlvyrntejejoinuotdaspfckguskskosssaelpynamnswseyctlklfaonb
│ CAROL_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxihksbbgdcsfypedrwpmdpkloeespbyflwlgtgtynhduyeclyvybtkewysbsnnymwoyaylstpsotansgylftanshfhdcxqzaavtmorloshpfmenimcefdclynlydedpfdztcfvsqzlbbbahjlfhltknmshfhktansgrhdcxcewevshesreclkbgfglyhkettprybbbwjelndpmohgtbjlrezmfeaycytewycmecoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzbgeththhosaykstiuocfmkkomweerlltprrtrhdrdshytiuygraawfmuwytamhbbsnbdvychjttscwkpbwkplbpkrtlntszeuoenbyzcmhgwwpttbdwspfwmdmlaotmhpkjldyjn
```

## Provisioning XID for Dan

Generate Dan's key material, a private XID document (for owner use), and a signed public XID document (for participants).

```
DAN_PRVKEYS=$(envelope generate prvkeys)
echo "DAN_PRVKEYS=$DAN_PRVKEYS"
DAN_PUBKEYS=$(envelope generate pubkeys "$DAN_PRVKEYS")
echo "DAN_PUBKEYS=$DAN_PUBKEYS"
DAN_OWNER_DOC=$(envelope xid new --nickname Dan --sign inception "$DAN_PRVKEYS")
echo "DAN_OWNER_DOC=$DAN_OWNER_DOC"
DAN_SIGNED_DOC=$(envelope xid new --nickname Dan --private omit --sign inception "$DAN_PRVKEYS")
echo "DAN_SIGNED_DOC=$DAN_SIGNED_DOC"

│ DAN_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxbzbdinbdremuwpftdwpruydmyndecahtuoweeyvegyonispeaecsrfcxrnhslgwmtansgehdcxskatdaguynfzfhisynhpgwsoihpfproesrkoctdibbsogslgnefxvswlvlsssprfnyoynspy
│ DAN_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxdkoxvwahztpkjlidjtdwzceefewyladtgaemnsbetanyvdonhgqzytdpmhylnbfdtansgrhdcxzsgmnefmjkhkiasefhguhghhdpaopessldaerhettywpgozmamimehcsdtnltkflwdndbymn
│ DAN_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxjthfmotdoerhttlustqdaokpdisosbnngouyvwdsckptrojklsndiyuedarkayoeoyaylrtpsotansgylftanshfhdcxdkoxvwahztpkjlidjtdwzceefewyladtgaemnsbetanyvdonhgqzytdpmhylnbfdtansgrhdcxzsgmnefmjkhkiasefhguhghhdpaopessldaerhettywpgozmamimehcsdtnltkfloycsfncsfglfoycsfptpsotansgtlftansgohdcxbzbdinbdremuwpftdwpruydmyndecahtuoweeyvegyonispeaecsrfcxrnhslgwmtansgehdcxskatdaguynfzfhisynhpgwsoihpfproesrkoctdibbsogslgnefxvswlvlsssprfoybstpsotansgmhdcxjzhyoxgybkhetdvwmtjedeotgecnketbknehltknataxsrvawdtdisgtaswedscpoycscstpsoiafyhsjtoyaxtpsotansghhdfzprdahlsbftkolshepepmwnlpdtttgtdntiuteeiameecfmjtnbahldbtjlhsbzfyoeatrhdrptihvsryvwskjscmfpwzespydtcxmdktmwkigwlehyckwklbhtutiepfzmnlkeeo
│ DAN_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxjthfmotdoerhttlustqdaokpdisosbnngouyvwdsckptrojklsndiyuedarkayoeoyaylstpsotansgylftanshfhdcxdkoxvwahztpkjlidjtdwzceefewyladtgaemnsbetanyvdonhgqzytdpmhylnbfdtansgrhdcxzsgmnefmjkhkiasefhguhghhdpaopessldaerhettywpgozmamimehcsdtnltkfloycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfzhgpsrflerlcetsutmkureenyehoxotwlwtisceueetdiadksylssonltbdwljzgdkikbjokshgcyptpfaeqdwegrmdjetirfsbiowpeytilkhdmwjefpmonsfylydrhpwepdjyuy
```

## Building Alice's registry

Set Alice as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
ALICE_REGISTRY=demo/alice-registry.json
frost owner set --registry "$ALICE_REGISTRY" "$ALICE_OWNER_DOC"
frost participant add --registry "$ALICE_REGISTRY" "$BOB_SIGNED_DOC" Bob
frost participant add --registry "$ALICE_REGISTRY" "$CAROL_SIGNED_DOC" Carol
frost participant add --registry "$ALICE_REGISTRY" "$DAN_SIGNED_DOC" Dan
cat "$ALICE_REGISTRY"

│ {
│   "owner": {
│     "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxfpfrluaymknymkoeutndsapfadpkfnaasggobgmoaocltbfnltynzeceyksflpetoyaylrtpsotansgylftanshfhdcxcwoxjoidbafdwfjydposesrddinypthlclssvedsemknghvwchpkenskksmwtsjotansgrhdcxjzieihkkftaypmqzjnbdwmpklptbhyjzjlbnbdbzhhmocwmohhzehezotbdkfwdaoycsfncsfgoycscstpsoihfpjziniaihlfoycsfptpsotansgtlftansgohdcxsefnoxfyutgrurytrtbgoxcfeooefsjtyasksftaksdtdkhdetayvlzcpmsemsehtansgehdcxskwpftqzihhlettlhevefpjslrptzepahfsnpegstivebymnbtsaqdztkosetamooybstpsotansgmhdcxlgnnsspahddkldgyuewmfdleswqdveinlefptsftwftkdnnnhgfwneecpkaddisboyaxtpsotansghhdfzhdstbtfegaclplgsiooldasfdsgabdjzfgwloxnshnytglaycycnfyctweaahybkdliohltymuhgspvesandgalpioldgrfndncmfxmtroctrtjtgoytswlnbnwpvyhhnegyoskg"
│   },
│   "participants": {
│     "ur:xid/hdcxhsktpyhseebasnenjkisaejnjohpplsackimwyktcmqzgdjnwfhlmnhtdshdbgwdehjkwfkg": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxhsktpyhseebasnenjkisaejnjohpplsackimwyktcmqzgdjnwfhlmnhtdshdbgwdoyaylstpsotansgylftanshfhdcxcahkhtfdgopakpbzvardwmcletldsbltzshkteehonvyksollohfkelomhskkssbtansgrhdcxparpkbjtspdretaavswdbghdhhfxkbkesawnflykidoxgsldbzaylfsaiewsrngooycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzgwpdytrhhfjyoymotsykbthfuturjolgctpewfgtfzsnkthghkrskbzeurkgnbonsauecfjycnkbtantlpwztkgraohkgwlsluwzhfpafhrsdrsrynmejnvljndnchcplydeimnl",
│       "pet_name": "Bob"
│     },
│     "ur:xid/hdcxihksbbgdcsfypedrwpmdpkloeespbyflwlgtgtynhduyeclyvybtkewysbsnnymwflswfrls": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxihksbbgdcsfypedrwpmdpkloeespbyflwlgtgtynhduyeclyvybtkewysbsnnymwoyaylstpsotansgylftanshfhdcxqzaavtmorloshpfmenimcefdclynlydedpfdztcfvsqzlbbbahjlfhltknmshfhktansgrhdcxcewevshesreclkbgfglyhkettprybbbwjelndpmohgtbjlrezmfeaycytewycmecoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzbgeththhosaykstiuocfmkkomweerlltprrtrhdrdshytiuygraawfmuwytamhbbsnbdvychjttscwkpbwkplbpkrtlntszeuoenbyzcmhgwwpttbdwspfwmdmlaotmhpkjldyjn",
│       "pet_name": "Carol"
│     },
│     "ur:xid/hdcxjthfmotdoerhttlustqdaokpdisosbnngouyvwdsckptrojklsndiyuedarkayoetycsclpm": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxjthfmotdoerhttlustqdaokpdisosbnngouyvwdsckptrojklsndiyuedarkayoeoyaylstpsotansgylftanshfhdcxdkoxvwahztpkjlidjtdwzceefewyladtgaemnsbetanyvdonhgqzytdpmhylnbfdtansgrhdcxzsgmnefmjkhkiasefhguhghhdpaopessldaerhettywpgozmamimehcsdtnltkfloycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfzhgpsrflerlcetsutmkureenyehoxotwlwtisceueetdiadksylssonltbdwljzgdkikbjokshgcyptpfaeqdwegrmdjetirfsbiowpeytilkhdmwjefpmonsfylydrhpwepdjyuy",
│       "pet_name": "Dan"
│     }
│   }
│ }
```

## Building Bob's registry

Set Bob as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
BOB_REGISTRY=demo/bob-registry.json
frost owner set --registry "$BOB_REGISTRY" "$BOB_OWNER_DOC"
frost participant add --registry "$BOB_REGISTRY" "$ALICE_SIGNED_DOC" Alice
frost participant add --registry "$BOB_REGISTRY" "$CAROL_SIGNED_DOC" Carol
frost participant add --registry "$BOB_REGISTRY" "$DAN_SIGNED_DOC" Dan
cat "$BOB_REGISTRY"

│ {
│   "owner": {
│     "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxhsktpyhseebasnenjkisaejnjohpplsackimwyktcmqzgdjnwfhlmnhtdshdbgwdoyaylrtpsotansgylftanshfhdcxcahkhtfdgopakpbzvardwmcletldsbltzshkteehonvyksollohfkelomhskkssbtansgrhdcxparpkbjtspdretaavswdbghdhhfxkbkesawnflykidoxgsldbzaylfsaiewsrngooycsfncsfglfoycsfptpsotansgtlftansgohdcxurhphkdrbwcwrknebyondpryprdehtdplygyneosietpylgwnbfwwnaehddabdtptansgehdcxvthgswtpfyqzsakpjnkpswrelrnngwbgveindpvytlwsmhoerhnddkksvwecflosoybstpsotansgmhdcxuewdbegujnwkcflnssgdhgtsmnclcpoycawprkadrylfbsoelffnttylpaaaahckoycscstpsoiafwjlidoyaxtpsotansghhdfzbtnnflcxmhynihamtnttprrfjphnkbztdsndjnfhimmsatswembysafgatahlotytlttcffgyttosesflaghmhbybthtjkbdfhwnbbvdurgutlhscenewsflrpbsfzattiidcltk"
│   },
│   "participants": {
│     "ur:xid/hdcxfpfrluaymknymkoeutndsapfadpkfnaasggobgmoaocltbfnltynzeceyksflpetkklpeovl": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxfpfrluaymknymkoeutndsapfadpkfnaasggobgmoaocltbfnltynzeceyksflpetoyaylstpsotansgylftanshfhdcxcwoxjoidbafdwfjydposesrddinypthlclssvedsemknghvwchpkenskksmwtsjotansgrhdcxjzieihkkftaypmqzjnbdwmpklptbhyjzjlbnbdbzhhmocwmohhzehezotbdkfwdaoycsfncsfgoycscstpsoihfpjziniaihoyaxtpsotansghhdfzstswwkfnhycfaecktbzehgwnssgypswewshsbgjnfnzecxhgvwzevtjkhtstdioedlsemdspimdkstvewyfzgddkiychjpmytoclldspiycxhdrtdmtnehgradcapkdyrfjevoax",
│       "pet_name": "Alice"
│     },
│     "ur:xid/hdcxihksbbgdcsfypedrwpmdpkloeespbyflwlgtgtynhduyeclyvybtkewysbsnnymwflswfrls": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxihksbbgdcsfypedrwpmdpkloeespbyflwlgtgtynhduyeclyvybtkewysbsnnymwoyaylstpsotansgylftanshfhdcxqzaavtmorloshpfmenimcefdclynlydedpfdztcfvsqzlbbbahjlfhltknmshfhktansgrhdcxcewevshesreclkbgfglyhkettprybbbwjelndpmohgtbjlrezmfeaycytewycmecoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzbgeththhosaykstiuocfmkkomweerlltprrtrhdrdshytiuygraawfmuwytamhbbsnbdvychjttscwkpbwkplbpkrtlntszeuoenbyzcmhgwwpttbdwspfwmdmlaotmhpkjldyjn",
│       "pet_name": "Carol"
│     },
│     "ur:xid/hdcxjthfmotdoerhttlustqdaokpdisosbnngouyvwdsckptrojklsndiyuedarkayoetycsclpm": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxjthfmotdoerhttlustqdaokpdisosbnngouyvwdsckptrojklsndiyuedarkayoeoyaylstpsotansgylftanshfhdcxdkoxvwahztpkjlidjtdwzceefewyladtgaemnsbetanyvdonhgqzytdpmhylnbfdtansgrhdcxzsgmnefmjkhkiasefhguhghhdpaopessldaerhettywpgozmamimehcsdtnltkfloycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfzhgpsrflerlcetsutmkureenyehoxotwlwtisceueetdiadksylssonltbdwljzgdkikbjokshgcyptpfaeqdwegrmdjetirfsbiowpeytilkhdmwjefpmonsfylydrhpwepdjyuy",
│       "pet_name": "Dan"
│     }
│   }
│ }
```

## Building Carol's registry

Set Carol as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
CAROL_REGISTRY=demo/carol-registry.json
frost owner set --registry "$CAROL_REGISTRY" "$CAROL_OWNER_DOC"
frost participant add --registry "$CAROL_REGISTRY" "$ALICE_SIGNED_DOC" Alice
frost participant add --registry "$CAROL_REGISTRY" "$BOB_SIGNED_DOC" Bob
frost participant add --registry "$CAROL_REGISTRY" "$DAN_SIGNED_DOC" Dan
cat "$CAROL_REGISTRY"

│ {
│   "owner": {
│     "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxihksbbgdcsfypedrwpmdpkloeespbyflwlgtgtynhduyeclyvybtkewysbsnnymwoyaylrtpsotansgylftanshfhdcxqzaavtmorloshpfmenimcefdclynlydedpfdztcfvsqzlbbbahjlfhltknmshfhktansgrhdcxcewevshesreclkbgfglyhkettprybbbwjelndpmohgtbjlrezmfeaycytewycmecoycsfncsfglfoycsfptpsotansgtlftansgohdcxlsrsztrsrktbbkwygyfdfmjtjsssbtdidyonmwrkdakogucagdfgtnmkcmsapdehtansgehdcxlprtbzdspyvltoiofychckvegwghprlygatkaopftybniapdrspfbamyvyiefmweoybstpsotansgmhdcxlohtwehehtsgadbdfhnsmwwerdwldygosspkkpweaxsnnbmybwglrhglvlcxglhhoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzeefecfmhdibalybtytcsfhrefrjkfemhbyamckdnrhlomwdlwefdwleogyoxbadnkpotishscpesmtrlvyrntejejoinuotdaspfckguskskosssaelpynamnswseyctlklfaonb"
│   },
│   "participants": {
│     "ur:xid/hdcxfpfrluaymknymkoeutndsapfadpkfnaasggobgmoaocltbfnltynzeceyksflpetkklpeovl": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxfpfrluaymknymkoeutndsapfadpkfnaasggobgmoaocltbfnltynzeceyksflpetoyaylstpsotansgylftanshfhdcxcwoxjoidbafdwfjydposesrddinypthlclssvedsemknghvwchpkenskksmwtsjotansgrhdcxjzieihkkftaypmqzjnbdwmpklptbhyjzjlbnbdbzhhmocwmohhzehezotbdkfwdaoycsfncsfgoycscstpsoihfpjziniaihoyaxtpsotansghhdfzstswwkfnhycfaecktbzehgwnssgypswewshsbgjnfnzecxhgvwzevtjkhtstdioedlsemdspimdkstvewyfzgddkiychjpmytoclldspiycxhdrtdmtnehgradcapkdyrfjevoax",
│       "pet_name": "Alice"
│     },
│     "ur:xid/hdcxhsktpyhseebasnenjkisaejnjohpplsackimwyktcmqzgdjnwfhlmnhtdshdbgwdehjkwfkg": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxhsktpyhseebasnenjkisaejnjohpplsackimwyktcmqzgdjnwfhlmnhtdshdbgwdoyaylstpsotansgylftanshfhdcxcahkhtfdgopakpbzvardwmcletldsbltzshkteehonvyksollohfkelomhskkssbtansgrhdcxparpkbjtspdretaavswdbghdhhfxkbkesawnflykidoxgsldbzaylfsaiewsrngooycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzgwpdytrhhfjyoymotsykbthfuturjolgctpewfgtfzsnkthghkrskbzeurkgnbonsauecfjycnkbtantlpwztkgraohkgwlsluwzhfpafhrsdrsrynmejnvljndnchcplydeimnl",
│       "pet_name": "Bob"
│     },
│     "ur:xid/hdcxjthfmotdoerhttlustqdaokpdisosbnngouyvwdsckptrojklsndiyuedarkayoetycsclpm": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxjthfmotdoerhttlustqdaokpdisosbnngouyvwdsckptrojklsndiyuedarkayoeoyaylstpsotansgylftanshfhdcxdkoxvwahztpkjlidjtdwzceefewyladtgaemnsbetanyvdonhgqzytdpmhylnbfdtansgrhdcxzsgmnefmjkhkiasefhguhghhdpaopessldaerhettywpgozmamimehcsdtnltkfloycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfzhgpsrflerlcetsutmkureenyehoxotwlwtisceueetdiadksylssonltbdwljzgdkikbjokshgcyptpfaeqdwegrmdjetirfsbiowpeytilkhdmwjefpmonsfylydrhpwepdjyuy",
│       "pet_name": "Dan"
│     }
│   }
│ }
```

## Building Dan's registry

Set Dan as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
DAN_REGISTRY=demo/dan-registry.json
frost owner set --registry "$DAN_REGISTRY" "$DAN_OWNER_DOC"
frost participant add --registry "$DAN_REGISTRY" "$ALICE_SIGNED_DOC" Alice
frost participant add --registry "$DAN_REGISTRY" "$BOB_SIGNED_DOC" Bob
frost participant add --registry "$DAN_REGISTRY" "$CAROL_SIGNED_DOC" Carol
cat "$DAN_REGISTRY"

│ {
│   "owner": {
│     "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxjthfmotdoerhttlustqdaokpdisosbnngouyvwdsckptrojklsndiyuedarkayoeoyaylrtpsotansgylftanshfhdcxdkoxvwahztpkjlidjtdwzceefewyladtgaemnsbetanyvdonhgqzytdpmhylnbfdtansgrhdcxzsgmnefmjkhkiasefhguhghhdpaopessldaerhettywpgozmamimehcsdtnltkfloycsfncsfglfoycsfptpsotansgtlftansgohdcxbzbdinbdremuwpftdwpruydmyndecahtuoweeyvegyonispeaecsrfcxrnhslgwmtansgehdcxskatdaguynfzfhisynhpgwsoihpfproesrkoctdibbsogslgnefxvswlvlsssprfoybstpsotansgmhdcxjzhyoxgybkhetdvwmtjedeotgecnketbknehltknataxsrvawdtdisgtaswedscpoycscstpsoiafyhsjtoyaxtpsotansghhdfzprdahlsbftkolshepepmwnlpdtttgtdntiuteeiameecfmjtnbahldbtjlhsbzfyoeatrhdrptihvsryvwskjscmfpwzespydtcxmdktmwkigwlehyckwklbhtutiepfzmnlkeeo"
│   },
│   "participants": {
│     "ur:xid/hdcxfpfrluaymknymkoeutndsapfadpkfnaasggobgmoaocltbfnltynzeceyksflpetkklpeovl": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxfpfrluaymknymkoeutndsapfadpkfnaasggobgmoaocltbfnltynzeceyksflpetoyaylstpsotansgylftanshfhdcxcwoxjoidbafdwfjydposesrddinypthlclssvedsemknghvwchpkenskksmwtsjotansgrhdcxjzieihkkftaypmqzjnbdwmpklptbhyjzjlbnbdbzhhmocwmohhzehezotbdkfwdaoycsfncsfgoycscstpsoihfpjziniaihoyaxtpsotansghhdfzstswwkfnhycfaecktbzehgwnssgypswewshsbgjnfnzecxhgvwzevtjkhtstdioedlsemdspimdkstvewyfzgddkiychjpmytoclldspiycxhdrtdmtnehgradcapkdyrfjevoax",
│       "pet_name": "Alice"
│     },
│     "ur:xid/hdcxhsktpyhseebasnenjkisaejnjohpplsackimwyktcmqzgdjnwfhlmnhtdshdbgwdehjkwfkg": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxhsktpyhseebasnenjkisaejnjohpplsackimwyktcmqzgdjnwfhlmnhtdshdbgwdoyaylstpsotansgylftanshfhdcxcahkhtfdgopakpbzvardwmcletldsbltzshkteehonvyksollohfkelomhskkssbtansgrhdcxparpkbjtspdretaavswdbghdhhfxkbkesawnflykidoxgsldbzaylfsaiewsrngooycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzgwpdytrhhfjyoymotsykbthfuturjolgctpewfgtfzsnkthghkrskbzeurkgnbonsauecfjycnkbtantlpwztkgraohkgwlsluwzhfpafhrsdrsrynmejnvljndnchcplydeimnl",
│       "pet_name": "Bob"
│     },
│     "ur:xid/hdcxihksbbgdcsfypedrwpmdpkloeespbyflwlgtgtynhduyeclyvybtkewysbsnnymwflswfrls": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxihksbbgdcsfypedrwpmdpkloeespbyflwlgtgtynhduyeclyvybtkewysbsnnymwoyaylstpsotansgylftanshfhdcxqzaavtmorloshpfmenimcefdclynlydedpfdztcfvsqzlbbbahjlfhltknmshfhktansgrhdcxcewevshesreclkbgfglyhkettprybbbwjelndpmohgtbjlrezmfeaycytewycmecoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzbgeththhosaykstiuocfmkkomweerlltprrtrhdrdshytiuygraawfmuwytamhbbsnbdvychjttscwkpbwkplbpkrtlntszeuoenbyzcmhgwwpttbdwspfwmdmlaotmhpkjldyjn",
│       "pet_name": "Carol"
│     }
│   }
│ }
```

## Showing Alice's DKG invite (request envelope)

Create a 2-of-3 DKG invite for Bob, Carol, and Dan (from Alice's registry) and format the request envelope to inspect its structure.

```
ALICE_INVITE=$(frost dkg invite show --registry demo/alice-registry.json --min-signers 2 --charter "This group will authorize new club editions." Bob Carol Dan)
echo "${ALICE_INVITE}" | envelope format

│ request(ARID(179928db)) [
│     'body': «"dkgGroupInvite"» [
│         ❰"charter"❱: "This group will authorize new club editions."
│         ❰"minSigners"❱: 2
│         ❰"participant"❱: {
│             {
│                 XID(6177ab61) [
│                     'key': PublicKeys(a2a78e62, SigningPublicKey(6177ab61, SchnorrPublicKey(4df90697)), EncapsulationPublicKey(cb46c282, X25519PublicKey(cb46c282))) [
│                         'allow': 'All'
│                         'nickname': "Bob"
│                     ]
│                 ]
│             } [
│                 'signed': Signature
│             ]
│         } [
│             "response_arid": ENCRYPTED [
│                 'hasRecipient': SealedMessage
│             ]
│         ]
│         ❰"participant"❱: {
│             {
│                 XID(65781450) [
│                     'key': PublicKeys(6182e9df, SigningPublicKey(65781450, SchnorrPublicKey(4e4d3ad7)), EncapsulationPublicKey(1b2d4e8a, X25519PublicKey(1b2d4e8a))) [
│                         'allow': 'All'
│                         'nickname': "Carol"
│                     ]
│                 ]
│             } [
│                 'signed': Signature
│             ]
│         } [
│             "response_arid": ENCRYPTED [
│                 'hasRecipient': SealedMessage
│             ]
│         ]
│         ❰"participant"❱: {
│             {
│                 XID(6e5692d2) [
│                     'key': PublicKeys(a1d3c32a, SigningPublicKey(6e5692d2, SchnorrPublicKey(ae39af5c)), EncapsulationPublicKey(dfbf30a1, X25519PublicKey(dfbf30a1))) [
│                         'allow': 'All'
│                         'nickname': "Dan"
│                     ]
│                 ]
│             } [
│                 'signed': Signature
│             ]
│         } [
│             "response_arid": ENCRYPTED [
│                 'hasRecipient': SealedMessage
│             ]
│         ]
│         ❰"session"❱: ARID(4733c503)
│         ❰"validUntil"❱: 2025-11-22T11:53:57Z
│     ]
│     'date': 2025-11-22T10:53:57Z
│ ]
```

## Showing Alice's sealed DKG invite

Seal the 2-of-3 invite for Bob, Carol, and Dan and format the sealed envelope to view the encrypted recipient entries.

```
ALICE_INVITE_SEALED=$(frost dkg invite show --registry demo/alice-registry.json --sealed --min-signers 2 --charter "This group will authorize new club editions." Bob Carol Dan)
echo "${ALICE_INVITE_SEALED}" | envelope format
echo "${ALICE_INVITE_SEALED}" | envelope info

│ ENCRYPTED [
│     'hasRecipient': SealedMessage
│     'hasRecipient': SealedMessage
│     'hasRecipient': SealedMessage
│ ]
│ Format: ur:envelope
│ CBOR Size: 2626
│ Description: Gordian Envelope
```

