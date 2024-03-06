(module
	(import "mem" "memory" (memory 1))
	(import "mem" "print_digit" (func $print_digit (param i32)))

	(func $writeln (export "writeln") (param $addr i32)
		(block $body
		(block $line
		(block $zero
			(block $str
				(block $float
					(block $int
						(local.get $addr)
						(i32.load8_s offset=1)

						(br_table $int $float $str $body)
					)
					(local.get $addr)
					(i32.load offset=2)
					(call $write_i)
					(br $line)
				)
				(local.get $addr)
				(f32.load offset=2)
				(call $write_f)

				(br $line)
			)
			(br $body)
		)
		i32.const 0
		call $print_digit
		br $line
		)
		;; newline is 10
		i32.const -38
		call $print_digit
		br $body
		)
		return
	)

	(func $write_i (export "write_i") (param $num i32)
		(local $reversed i32)
		(block $zero
			(local.get $num)
			(i32.const 0)
			(i32.ne)

			(br_if $zero)
			(i32.const 0)
			(call $print_digit)
			(return)
		)
		
		i32.const 0
		local.set $reversed
		(block $end
			(loop $reverse
				(local.get $num)
				(i32.eqz)
				(br_if $end)

				(local.get $reversed)
				(i32.const 10)
				(i32.mul)
						
				(local.get $num)
				(i32.const 10)
				(i32.rem_u)

				(i32.add)
				(local.set $reversed)

				(local.get $num)
				(i32.const 10)
				(i32.div_s)
				(local.set $num)
				(br $reverse)
			)
		)

					
		(loop $loop
			;; digit = int  % 10
			(local.get $reversed)	
			(i32.const 10)
			(i32.rem_s)
						
			(call $print_digit)
			;; num = num / 10
			(local.get $reversed)
			(i32.const 10)
			(i32.div_s)
			(local.tee $reversed)
			(i32.const 0)
			(i32.ne)
			(br_if $loop)
		)
	)

	(func $write_f (export "write_f") (param $num f32)
		(local $trunc i32)
		local.get $num
		
		i32.trunc_f32_s
		local.tee $trunc
		
		call $write_i

		;; print . (46)
		i32.const -2
		call $print_digit
		
		local.get $num
		local.get $trunc
		f32.convert_i32_s
		f32.sub

		;; format here is the first 3 digits
		f32.const 100.0
		f32.mul
		i32.trunc_f32_s

		call $write_i
	)
)
