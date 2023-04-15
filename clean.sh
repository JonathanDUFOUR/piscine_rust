#!/usr/bin/fish

for mod in (ls -d mod*)
	for ex in (ls -d $mod/ex*)
		if test -f $ex/Cargo.toml
			cd $ex
			cargo clean
			cd -
		end
	end
end
