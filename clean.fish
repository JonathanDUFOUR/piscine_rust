#!/usr/bin/fish

for ex in ex{04, 05, 06, 07}
	cd module_00/$ex
	cargo clean
	cd -
end

for mod in module_{01, 02, 03, 04, 05, 06, 07}
	cd $mod
	cargo clean
	cd -
end
