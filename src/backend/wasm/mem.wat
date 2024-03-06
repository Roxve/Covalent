(module
	(import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
	(memory (export "memory") 1)
	;; init first page
	(data (i32.const 0) "\00")
	(data (i32.const 1) "\FF\FF")
	
	(global $ptr (export "ptr") (mut i32) (i32.const 0))
	(global $pages (export "pages") (mut i32) (i32.const 1))

	(func $size_of (export "size_of") (param $address i32) (result i32)
		;; its free address?
		local.get $address
		i32.load8_s
		i32.const 0
		i32.eq

		if (result i32)
			local.get $address
			i32.load16_u offset=1
		else
			;; 0-1 4byte types
			local.get $address
			i32.load8_s offset=1
			i32.const 1
			i32.le_s

			if (result i32)
				i32.const 6
			else
				i32.const -1
			end
		end
	)
	(func $move_ptr (export "move_ptr") (param $size i32) (result i32)
		(local $size_ptr i32)
		(local $size_block i32)
		(local $result i32)
		
		global.get $ptr
		call $size_of
		local.set $size_ptr
		
		global.get $ptr
		i32.load8_s
		i32.const 0
		i32.eq
		if (result i32)
	
			local.get $size_ptr
			local.get $size
			i32.eq
			if (result i32)
				global.get $ptr
			else
			;; size_ptr > size && (size_ptr - 3) >= size
			local.get $size_ptr
			local.get $size
			i32.ge_s

			local.get $size_ptr
			i32.const 3
			i32.sub
			local.get $size
			i32.ge_s
			i32.and
			if (result i32)
				global.get $ptr
				local.set $result
				;; we divide our block the new block is going to be free and its size is a u16
				;; size of our new block
				local.get $size_ptr
				local.get $size
				i32.sub
				local.set $size_block
				;; add size to ptr and create new block
				global.get $ptr
				local.get $size
				i32.add
				global.set $ptr

				global.get $ptr
				local.get $size_block
				i32.store16 offset=1
				;; ;; make sure its free
				;; global.get $ptr
				;; i32.const 0
				;; i32.store8
			
				local.get $result
				local.get $size
				i32.store16 offset=1

				local.get $result
			else
			;; we gotta find a new block
			global.get $ptr
			local.get $size_ptr
			i32.add
			global.set $ptr
			
			local.get $size
			call $move_ptr
		end
		end
		else
		global.get $ptr
		local.get $size_ptr
		i32.add
		global.set $ptr
		
		local.get $size
		call $move_ptr
		end
	)
	;; type-alloc
	(func $talloc (export "talloc") (param $ty i32) (result i32)
		(local $result i32)
		(local $address i32)
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
			i32.const 6
			call $move_ptr
			local.set $address

			local.get $address
			i32.const 1
			
			i32.store8

			local.get $address
			local.get $ty
			i32.store8 offset=1

			local.get $address
			local.set $result

	
			local.get $result
		else
			i32.const -1
		end
	)
	(func $mk_int (export "mk_int") (param $val i32) (result i32)
		(local $res i32)
		i32.const 0
		call $talloc
		local.tee $res
		
		local.get $val
		i32.store offset=2

		local.get $res
	)
	(func $mk_float (export "mk_float") (param $val f32) (result i32)
		(local $res i32)
		i32.const 1
		call $talloc
		local.tee $res
		
		local.get $val
		f32.store offset=2
		local.get $res
	)
	(func $addrfree (export "addrfree") (param $address i32)
		(local $size i32)
		(local $free i32)
		local.get $address
		call $size_of
		local.set $size

		;; overwrite the is_used and the next two bytes
		local.get $address
		i32.const 0
		i32.store8
		
		local.get $address
		local.get $size
		i32.store16 offset=1

		local.get $address
		global.set $ptr

		;; overwrite everything
				
		local.get $size
		i32.const 3
		i32.sub
		local.tee $free
		i32.eqz
		if
		return
		else
		
		local.get $address
		i32.const 2
		i32.add
		local.set $address
		(loop $loop
			(local.get $address)
			(i32.const 1)
			(i32.add)
			(local.tee $address)
			(i32.const 0)
			(i32.store8)
			
			(local.get $free)
			(i32.const 1)
			(i32.sub)
			(local.tee $free)
			(i32.const 0)
			(i32.ne)
			(br_if $loop)
		)
		end
		
	)
	(func $print_digit (export "print_digit") (param $digit i32)
		(local $old_ptr i32)
		global.get $ptr
		local.set $old_ptr
		;; go to area with 3+1+8+4 bytes (info + iovec struct + digit + written
		i32.const 16
		call $move_ptr
		;; info
		global.get $ptr
		i32.const 0
		i32.store8

		global.get $ptr
		i32.const 16
		i32.store16 offset=1

		global.get $ptr
		i32.const 48 ;; digit to ascii
		local.get $digit
		i32.add
		i32.store8 offset=3

		global.get $ptr
		global.get $ptr
		i32.const 3
		i32.add
		
		i32.store offset=4

		global.get $ptr
		i32.const 1
		i32.store offset=8

		i32.const 1 ;; write
		global.get $ptr
		i32.const 4 ;; ivoec
		i32.add
		i32.const 1
		
		global.get $ptr ;; write to
		i32.const 12
		i32.add

		call $fd_write
		drop

		;; set as free
		global.get $ptr
		i32.const 0
		i32.store8
		;; clean ptr
		global.get $ptr
		call $addrfree
		drop
		local.get $old_ptr
		global.set $ptr
	)
)
