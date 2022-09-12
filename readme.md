
# Showcase of a potential bug?

When generating a wallet from an extended private key (xprv) a wallet generated using bdk>19 is different to a wallet generated using bdk=19.

# Run with

```bash
cargo run
```

# Sample output

Unless someone stole the money on the above key, this is the expected output

```bash
BDK 018 name=`tenut4tc6dkakqmq`, balance=`1778469`, address_index_0=`tb1q9pwdmea68rpl652k69v99jxqzfq8sc28y5pkcm`, address_index_1=`tb1q09hdfahnvz52kmg9a4fu0d2nflhaxympxq9uns`, last_unused=`tb1q09hdfahnvz52kmg9a4fu0d2nflhaxympxq9uns`
BDK 019 name=`tenut4tc6dkakqmq`, balance=`1778469`, address_index_0=`tb1q9pwdmea68rpl652k69v99jxqzfq8sc28y5pkcm`, address_index_1=`tb1q09hdfahnvz52kmg9a4fu0d2nflhaxympxq9uns`, last_unused=`tb1q09hdfahnvz52kmg9a4fu0d2nflhaxympxq9uns`
BDK 020 name=`q6whqwp03wtkam3h`, balance=`0`, address_index_0=`tb1qasay4ft0djnka9mjwekxtlqrm8088zrtf2la9h`, address_index_1=`tb1qsnqkgfwaqcc428mth3qn397phwvrdm2qtlypka`, last_unused=`tb1qasay4ft0djnka9mjwekxtlqrm8088zrtf2la9h`
BDK 021 name=`q6whqwp03wtkam3h`, balance=`0`, address_index_0=`tb1qasay4ft0djnka9mjwekxtlqrm8088zrtf2la9h`, address_index_1=`tb1qsnqkgfwaqcc428mth3qn397phwvrdm2qtlypka`, last_unused=`tb1qasay4ft0djnka9mjwekxtlqrm8088zrtf2la9h`
BDK 022 name=`q6whqwp03wtkam3h`, balance=`0`, address_index_0=`tb1qasay4ft0djnka9mjwekxtlqrm8088zrtf2la9h`, address_index_1=`tb1qsnqkgfwaqcc428mth3qn397phwvrdm2qtlypka`, last_unused=`tb1qasay4ft0djnka9mjwekxtlqrm8088zrtf2la9h`

```

# Tested with
`rustc 1.62.0 (a8314ef7d 2022-06-27)`
`cargo cargo 1.62.0 (a748cf5a3 2022-06-08)`