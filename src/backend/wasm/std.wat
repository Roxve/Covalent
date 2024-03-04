(module
	(import "mem" "memory" (memory 1))
	(import "mem" "print_digit" (func $print_digit (param i32)))

	(func $writeln (export "writeln") (param $addr i32)
		(local $num i32)
		(local $reversed i32)
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
					(local.tee $num)
					(i32.eqz)
					(br_if $zero)

					
					
					(i32.const 0)
					(local.set $reversed)
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
					))

					
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
					(br $line)
				)
				(br $body)
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
)
