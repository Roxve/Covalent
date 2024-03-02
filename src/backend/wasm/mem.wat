(module
	(memory (export "memory") 1)
	(global $ptr (export "ptr") (mut i32) (i32.const 0))
	(global $pages (export "pages") (mut i32) (i32.const 1))
	
	(func $move_ptr (export "move_ptr") (param $size i32)
		local.get $size
		global.get $ptr
		i32.add
		global.set $ptr
		
		global.get $ptr
		i32.load8_s
		i32.const 0
		i32.eq
		if
			return
		else
			i32.const 6
			call $move_ptr
		end
	)
	;; type-alloc
	(func (export "talloc") (param $ty i32) (result i32)
		(local $result i32)
		;;right now there is two 6 byte types
		local.get $ty
		i32.const 1
		i32.le_s
		if (result i32)
			;; first check if we need to grow memory 
			global.get $ptr
			global.get $pages
			i32.const 65536
			i32.mul
			i32.ge_s

			if
				i32.const 1
				global.get $pages
				i32.add
				global.set $pages 

				i32.const 1
				memory.grow
				drop
			end
			;; ptr should point to an area of free 6 bytes...
			global.get $ptr
			i32.const 1
			
			i32.store8

			global.get $ptr
			local.get $ty
			i32.store8 offset=1

			;; now we should move our $ptr to an empty area of 6 bytes
			global.get $ptr
			local.set $result

			i32.const 6
			call $move_ptr

			
			local.get $result
		else
			i32.const -1
		end
	)
	(func $addrfree (export "addrfree") (param $address i32)
		local.get $address
		i32.load8_s offset=1
		i32.const 1
		i32.le_s
		if
			;; overwrite is_used type and data
			local.get $address
			i32.const 0
			i32.store8
			
			local.get $address
			i32.const 0
			i32.store8 offset=1

			local.get $address
			i32.const 0
			i32.store offset=2

			local.get $address
			global.set $ptr
			return
		else
			return
		end
	)
)
