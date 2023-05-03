; ASM of deserialize_struct called from main.rs/lib.rs (looks the same) with lto = "fat"
00000000000106c0 <_ZN21rust_lib_closure_test18deserialize_struct17h4cacb9a8beb21afbE>:
   106c0:	48 8b 47 08          	mov    0x8(%rdi),%rax
   106c4:	48 83 f8 02          	cmp    $0x2,%rax
   106c8:	76 47                	jbe    10711 <_ZN21rust_lib_closure_test18deserialize_struct17h4cacb9a8beb21afbE+0x51>
   106ca:	48 8b 0f             	mov    (%rdi),%rcx
   106cd:	44 0f b6 01          	movzbl (%rcx),%r8d
   106d1:	48 8d 51 01          	lea    0x1(%rcx),%rdx
   106d5:	4c 8d 48 ff          	lea    -0x1(%rax),%r9
   106d9:	48 89 17             	mov    %rdx,(%rdi)
   106dc:	4c 89 4f 08          	mov    %r9,0x8(%rdi)
   106e0:	44 88 06             	mov    %r8b,(%rsi)
   106e3:	44 0f b6 41 01       	movzbl 0x1(%rcx),%r8d
   106e8:	48 8d 51 02          	lea    0x2(%rcx),%rdx
   106ec:	4c 8d 48 fe          	lea    -0x2(%rax),%r9
   106f0:	48 89 17             	mov    %rdx,(%rdi)
   106f3:	4c 89 4f 08          	mov    %r9,0x8(%rdi)
   106f7:	44 88 46 01          	mov    %r8b,0x1(%rsi)
   106fb:	0f b6 51 02          	movzbl 0x2(%rcx),%edx
   106ff:	48 83 c1 03          	add    $0x3,%rcx
   10703:	48 83 c0 fd          	add    $0xfffffffffffffffd,%rax
   10707:	48 89 0f             	mov    %rcx,(%rdi)
   1070a:	48 89 47 08          	mov    %rax,0x8(%rdi)
   1070e:	88 56 02             	mov    %dl,0x2(%rsi)
   10711:	c3                   	ret
   10712:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   10719:	00 00 00
   1071c:	0f 1f 40 00          	nopl   0x0(%rax)