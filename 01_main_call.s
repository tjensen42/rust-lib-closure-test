0000000000008b90 <_ZN21rust_lib_closure_test18deserialize_struct17h78300619e9e5bdc1E>:
    8b90:	41 57                	push   %r15
    8b92:	41 56                	push   %r14
    8b94:	53                   	push   %rbx
    8b95:	48 83 7f 08 02       	cmpq   $0x2,0x8(%rdi)
    8b9a:	76 26                	jbe    8bc2 <_ZN21rust_lib_closure_test18deserialize_struct17h78300619e9e5bdc1E+0x32>
    8b9c:	49 89 d7             	mov    %rdx,%r15
    8b9f:	49 89 f6             	mov    %rsi,%r14
    8ba2:	48 89 fb             	mov    %rdi,%rbx
    8ba5:	ff 12                	call   *(%rdx)
    8ba7:	48 89 df             	mov    %rbx,%rdi
    8baa:	4c 89 f6             	mov    %r14,%rsi
    8bad:	41 ff 57 08          	call   *0x8(%r15)
    8bb1:	49 8b 47 10          	mov    0x10(%r15),%rax
    8bb5:	48 89 df             	mov    %rbx,%rdi
    8bb8:	4c 89 f6             	mov    %r14,%rsi
    8bbb:	5b                   	pop    %rbx
    8bbc:	41 5e                	pop    %r14
    8bbe:	41 5f                	pop    %r15
    8bc0:	ff e0                	jmp    *%rax
    8bc2:	5b                   	pop    %rbx
    8bc3:	41 5e                	pop    %r14
    8bc5:	41 5f                	pop    %r15
    8bc7:	c3                   	ret
    8bc8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
    8bcf:	00 