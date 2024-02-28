git ls-files src/n*.rs | sed 's?^src/?mod ?g' | sed 's?\.rs?;?g' | sort -u
