(module
	(import "mem" "memory" (memory 1))
	(import "mem" "size_of" (func $size_of (param i32) (result i32)))
	(import "mem" "move_ptr" (func $move_ptr (param i32) (result i32)))
	(import "mem" "pos" (func $ptr (result i32)))
	(import "mem" "addrfree" (func $free (param i32)))
	(import "mem" "mk_int" (func $mk_int (param i32) (result i32)))
	(import "mem" "mk_float" (func $mk_float (param f32) (result i32)))
	(func $prepare (param $a i32) (result i32)
		(local $res i32)
		(local $size i32)
		(local $x i32)
		;; takes a and b makes new spaces for a and b instead of deleting them on use if use parameter says tho
		;; in the future it will do more...
		local.get $a
		i32.load8_s
		i32.const 3
		i32.eq
		if (result i32)
			local.get $a
			call $size_of
			local.tee $size
			
			call $move_ptr
			local.set $res

			(loop $loop
				(local.get $res)
				(local.get $a)
				(i32.load8_s)
				(i32.store8) 

				;; a++
				(local.get $a)
				(i32.const 1)
				(i32.add)
				(local.set $a)

				;; size--
				(local.get $size)
				(i32.const 1)
				(i32.sub)
				(local.set $size)

				(local.get $size)
				(i32.const 0)
				(i32.ne)
				(br_if $loop)
			)

			;; set use to 1
			local.get $res
			i32.const 1
			i32.store8

			local.get $res
		else
			local.get $a
		end
	)
	
	(func $__conv__ (export "__conv__") (param $a i32) (param $b i32) 		
		(local $type_a i32)
		(local $type_b i32)

		local.get $a
		i32.load8_s offset=1
		local.set $type_a
		
		local.get $b
		i32.load8_s offset=1
		local.set $type_b


		;; (type a != type b)
		local.get $type_a
		local.get $type_b
		i32.ne

		if 
			;; a == float && b == int

			local.get $type_a
			i32.const 1
			i32.eq

			local.get $type_b
			i32.eqz
			i32.and
			if 
				local.get $b
				;; type = float
				i32.const 1
				i32.store8 offset=1

				;; value = float
				local.get $b

				local.get $b
				i32.load offset=2
				f32.convert_i32_s

				f32.store offset=2

			end
			;; a == int && b == float

			local.get $type_b
			i32.const 1
			i32.eq

			local.get $type_a
			i32.eqz
			i32.and
			if 				
				local.get $a
				;; type = float
				i32.const 1
				i32.store8 offset=1

				;; value = float
				local.get $a

				local.get $a
				i32.load offset=2
				f32.convert_i32_s

				f32.store offset=2

			end
		else
			return
		end
		
	)
	(func $__add__(export "__add__") (param $a i32) (param $b i32) (result i32)
		(local $res i32)
		(local $ires i32)
		(local $fres f32)
		local.get $a
		call $prepare
		local.set $a
		
		local.get $b
		call $prepare
		local.set $b

		local.get $a
		local.get $b
		call $__conv__
		(block $default
			(block $str
				(block $float
					(block $int
						(local.get $a)
						(i32.load8_s offset=1)
						(br_table $int $float $str $default)
					)
					local.get $a
					i32.load offset=2

					local.get $b
					i32.load offset=2

					i32.add
					local.set $ires

					local.get $b
					call $free 
					local.get $a
					call $free

					local.get $ires
					call $mk_int
					local.set $res
					br $default
				)
					local.get $a
					f32.load offset=2

					local.get $b
					f32.load offset=2

					f32.add
					local.set $fres
					
					local.get $b
					call $free 
					local.get $a
					call $free
					
					local.get $fres

					call $mk_float
					local.set $res
					br $default
			)
			br $default
		)
		local.get $res
	)
)
