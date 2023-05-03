0000000000008c90 <_ZN21rust_lib_closure_test18deserialize_struct17h7cac8bcc2fad5118E>:
    8c90:	48 8b 47 08          	mov    0x8(%rdi),%rax
    8c94:	48 83 f8 02          	cmp    $0x2,%rax
    8c98:	76 25                	jbe    8cbf <_ZN21rust_lib_closure_test18deserialize_struct17h7cac8bcc2fad5118E+0x2f>
    8c9a:	48 8b 0f             	mov    (%rdi),%rcx
    8c9d:	0f b6 11             	movzbl (%rcx),%edx
    8ca0:	88 16                	mov    %dl,(%rsi)
    8ca2:	0f b6 51 01          	movzbl 0x1(%rcx),%edx
    8ca6:	88 56 01             	mov    %dl,0x1(%rsi)
    8ca9:	0f b6 51 02          	movzbl 0x2(%rcx),%edx
    8cad:	48 83 c1 03          	add    $0x3,%rcx
    8cb1:	48 83 c0 fd          	add    $0xfffffffffffffffd,%rax
    8cb5:	48 89 0f             	mov    %rcx,(%rdi)
    8cb8:	48 89 47 08          	mov    %rax,0x8(%rdi)
    8cbc:	88 56 02             	mov    %dl,0x2(%rsi)
    8cbf:	c3                   	ret