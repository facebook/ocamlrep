#pragma once

#if defined(__APPLE__)
#define _ecp_nistz256_point_double _p256_point_double
#define _ecp_nistz256_point_add _p256_point_add
#define _ecp_nistz256_point_add_affine _p256_point_add_affine
#define _ecp_nistz256_ord_mul_mont _p256_scalar_mul_mont
#define _ecp_nistz256_ord_sqr_mont _p256_scalar_sqr_rep_mont
#define _ecp_nistz256_mul_mont _p256_mul_mont
#define _ecp_nistz256_sqr_mont _p256_sqr_mont
#define _CRYPTO_memcmp _ring_core_0_17_5_CRYPTO_memcmp
#define _CRYPTO_poly1305_finish _ring_core_0_17_5_CRYPTO_poly1305_finish
#define _CRYPTO_poly1305_finish_neon _ring_core_0_17_5_CRYPTO_poly1305_finish_neon
#define _CRYPTO_poly1305_init _ring_core_0_17_5_CRYPTO_poly1305_init
#define _CRYPTO_poly1305_init_neon _ring_core_0_17_5_CRYPTO_poly1305_init_neon
#define _CRYPTO_poly1305_update _ring_core_0_17_5_CRYPTO_poly1305_update
#define _CRYPTO_poly1305_update_neon _ring_core_0_17_5_CRYPTO_poly1305_update_neon
#define _ChaCha20_ctr32 _ring_core_0_17_5_ChaCha20_ctr32
#define _LIMBS_add_mod _ring_core_0_17_5_LIMBS_add_mod
#define _LIMBS_are_even _ring_core_0_17_5_LIMBS_are_even
#define _LIMBS_are_zero _ring_core_0_17_5_LIMBS_are_zero
#define _LIMBS_equal _ring_core_0_17_5_LIMBS_equal
#define _LIMBS_equal_limb _ring_core_0_17_5_LIMBS_equal_limb
#define _LIMBS_less_than _ring_core_0_17_5_LIMBS_less_than
#define _LIMBS_less_than_limb _ring_core_0_17_5_LIMBS_less_than_limb
#define _LIMBS_reduce_once _ring_core_0_17_5_LIMBS_reduce_once
#define _LIMBS_select_512_32 _ring_core_0_17_5_LIMBS_select_512_32
#define _LIMBS_shl_mod _ring_core_0_17_5_LIMBS_shl_mod
#define _LIMBS_sub_mod _ring_core_0_17_5_LIMBS_sub_mod
#define _LIMBS_window5_split_window _ring_core_0_17_5_LIMBS_window5_split_window
#define _LIMBS_window5_unsplit_window _ring_core_0_17_5_LIMBS_window5_unsplit_window
#define _LIMB_shr _ring_core_0_17_5_LIMB_shr
#define _OPENSSL_armcap_P _ring_core_0_17_5_OPENSSL_armcap_P
#define _OPENSSL_cpuid_setup _ring_core_0_17_5_OPENSSL_cpuid_setup
#define _OPENSSL_ia32cap_P _ring_core_0_17_5_OPENSSL_ia32cap_P
#define _aes_hw_ctr32_encrypt_blocks _ring_core_0_17_5_aes_hw_ctr32_encrypt_blocks
#define _aes_hw_encrypt _ring_core_0_17_5_aes_hw_encrypt
#define _aes_hw_set_encrypt_key _ring_core_0_17_5_aes_hw_set_encrypt_key
#define _aes_nohw_ctr32_encrypt_blocks _ring_core_0_17_5_aes_nohw_ctr32_encrypt_blocks
#define _aes_nohw_encrypt _ring_core_0_17_5_aes_nohw_encrypt
#define _aes_nohw_set_encrypt_key _ring_core_0_17_5_aes_nohw_set_encrypt_key
#define _aesni_gcm_decrypt _ring_core_0_17_5_aesni_gcm_decrypt
#define _aesni_gcm_encrypt _ring_core_0_17_5_aesni_gcm_encrypt
#define _bn_from_montgomery_in_place _ring_core_0_17_5_bn_from_montgomery_in_place
#define _bn_gather5 _ring_core_0_17_5_bn_gather5
#define _bn_mul_mont _ring_core_0_17_5_bn_mul_mont
#define _bn_mul_mont_gather5 _ring_core_0_17_5_bn_mul_mont_gather5
#define _bn_neg_inv_mod_r_u64 _ring_core_0_17_5_bn_neg_inv_mod_r_u64
#define _bn_power5 _ring_core_0_17_5_bn_power5
#define _bn_scatter5 _ring_core_0_17_5_bn_scatter5
#define _bn_sqr8x_internal _ring_core_0_17_5_bn_sqr8x_internal
#define _bn_sqrx8x_internal _ring_core_0_17_5_bn_sqrx8x_internal
#define _bsaes_ctr32_encrypt_blocks _ring_core_0_17_5_bsaes_ctr32_encrypt_blocks
#define _bssl_constant_time_test_conditional_memcpy _ring_core_0_17_5_bssl_constant_time_test_conditional_memcpy
#define _bssl_constant_time_test_conditional_memxor _ring_core_0_17_5_bssl_constant_time_test_conditional_memxor
#define _bssl_constant_time_test_main _ring_core_0_17_5_bssl_constant_time_test_main
#define _chacha20_poly1305_open _ring_core_0_17_5_chacha20_poly1305_open
#define _chacha20_poly1305_seal _ring_core_0_17_5_chacha20_poly1305_seal
#define _fiat_curve25519_adx_mul _ring_core_0_17_5_fiat_curve25519_adx_mul
#define _fiat_curve25519_adx_square _ring_core_0_17_5_fiat_curve25519_adx_square
#define _gcm_ghash_avx _ring_core_0_17_5_gcm_ghash_avx
#define _gcm_ghash_clmul _ring_core_0_17_5_gcm_ghash_clmul
#define _gcm_ghash_neon _ring_core_0_17_5_gcm_ghash_neon
#define _gcm_gmult_clmul _ring_core_0_17_5_gcm_gmult_clmul
#define _gcm_gmult_neon _ring_core_0_17_5_gcm_gmult_neon
#define _gcm_init_avx _ring_core_0_17_5_gcm_init_avx
#define _gcm_init_clmul _ring_core_0_17_5_gcm_init_clmul
#define _gcm_init_neon _ring_core_0_17_5_gcm_init_neon
#define _k25519Precomp _ring_core_0_17_5_k25519Precomp
#define _limbs_mul_add_limb _ring_core_0_17_5_limbs_mul_add_limb
#define _little_endian_bytes_from_scalar _ring_core_0_17_5_little_endian_bytes_from_scalar
#define _ecp_nistz256_neg _ring_core_0_17_5_ecp_nistz256_neg
#define _ecp_nistz256_select_w5 _ring_core_0_17_5_ecp_nistz256_select_w5
#define _ecp_nistz256_select_w7 _ring_core_0_17_5_ecp_nistz256_select_w7
#define _p256_mul_mont _ring_core_0_17_5_p256_mul_mont
#define _p256_point_add _ring_core_0_17_5_p256_point_add
#define _p256_point_add_affine _ring_core_0_17_5_p256_point_add_affine
#define _p256_point_double _ring_core_0_17_5_p256_point_double
#define _p256_point_mul _ring_core_0_17_5_p256_point_mul
#define _p256_point_mul_base _ring_core_0_17_5_p256_point_mul_base
#define _p256_point_mul_base_vartime _ring_core_0_17_5_p256_point_mul_base_vartime
#define _p256_scalar_mul_mont _ring_core_0_17_5_p256_scalar_mul_mont
#define _p256_scalar_sqr_rep_mont _ring_core_0_17_5_p256_scalar_sqr_rep_mont
#define _p256_sqr_mont _ring_core_0_17_5_p256_sqr_mont
#define _p384_elem_div_by_2 _ring_core_0_17_5_p384_elem_div_by_2
#define _p384_elem_mul_mont _ring_core_0_17_5_p384_elem_mul_mont
#define _p384_elem_neg _ring_core_0_17_5_p384_elem_neg
#define _p384_elem_sub _ring_core_0_17_5_p384_elem_sub
#define _p384_point_add _ring_core_0_17_5_p384_point_add
#define _p384_point_double _ring_core_0_17_5_p384_point_double
#define _p384_point_mul _ring_core_0_17_5_p384_point_mul
#define _p384_scalar_mul_mont _ring_core_0_17_5_p384_scalar_mul_mont
#define _openssl_poly1305_neon2_addmulmod _ring_core_0_17_5_openssl_poly1305_neon2_addmulmod
#define _openssl_poly1305_neon2_blocks _ring_core_0_17_5_openssl_poly1305_neon2_blocks
#define _sha256_block_data_order _ring_core_0_17_5_sha256_block_data_order
#define _sha512_block_data_order _ring_core_0_17_5_sha512_block_data_order
#define _vpaes_ctr32_encrypt_blocks _ring_core_0_17_5_vpaes_ctr32_encrypt_blocks
#define _vpaes_encrypt _ring_core_0_17_5_vpaes_encrypt
#define _vpaes_encrypt_key_to_bsaes _ring_core_0_17_5_vpaes_encrypt_key_to_bsaes
#define _vpaes_set_encrypt_key _ring_core_0_17_5_vpaes_set_encrypt_key
#define _x25519_NEON _ring_core_0_17_5_x25519_NEON
#define _x25519_fe_invert _ring_core_0_17_5_x25519_fe_invert
#define _x25519_fe_isnegative _ring_core_0_17_5_x25519_fe_isnegative
#define _x25519_fe_mul_ttt _ring_core_0_17_5_x25519_fe_mul_ttt
#define _x25519_fe_neg _ring_core_0_17_5_x25519_fe_neg
#define _x25519_fe_tobytes _ring_core_0_17_5_x25519_fe_tobytes
#define _x25519_ge_double_scalarmult_vartime _ring_core_0_17_5_x25519_ge_double_scalarmult_vartime
#define _x25519_ge_frombytes_vartime _ring_core_0_17_5_x25519_ge_frombytes_vartime
#define _x25519_ge_scalarmult_base _ring_core_0_17_5_x25519_ge_scalarmult_base
#define _x25519_ge_scalarmult_base_adx _ring_core_0_17_5_x25519_ge_scalarmult_base_adx
#define _x25519_public_from_private_generic_masked _ring_core_0_17_5_x25519_public_from_private_generic_masked
#define _x25519_sc_mask _ring_core_0_17_5_x25519_sc_mask
#define _x25519_sc_muladd _ring_core_0_17_5_x25519_sc_muladd
#define _x25519_sc_reduce _ring_core_0_17_5_x25519_sc_reduce
#define _x25519_scalar_mult_adx _ring_core_0_17_5_x25519_scalar_mult_adx
#define _x25519_scalar_mult_generic_masked _ring_core_0_17_5_x25519_scalar_mult_generic_masked

#else
#define ecp_nistz256_point_double p256_point_double
#define ecp_nistz256_point_add p256_point_add
#define ecp_nistz256_point_add_affine p256_point_add_affine
#define ecp_nistz256_ord_mul_mont p256_scalar_mul_mont
#define ecp_nistz256_ord_sqr_mont p256_scalar_sqr_rep_mont
#define ecp_nistz256_mul_mont p256_mul_mont
#define ecp_nistz256_sqr_mont p256_sqr_mont
#define CRYPTO_memcmp ring_core_0_17_5_CRYPTO_memcmp
#define CRYPTO_poly1305_finish ring_core_0_17_5_CRYPTO_poly1305_finish
#define CRYPTO_poly1305_finish_neon ring_core_0_17_5_CRYPTO_poly1305_finish_neon
#define CRYPTO_poly1305_init ring_core_0_17_5_CRYPTO_poly1305_init
#define CRYPTO_poly1305_init_neon ring_core_0_17_5_CRYPTO_poly1305_init_neon
#define CRYPTO_poly1305_update ring_core_0_17_5_CRYPTO_poly1305_update
#define CRYPTO_poly1305_update_neon ring_core_0_17_5_CRYPTO_poly1305_update_neon
#define ChaCha20_ctr32 ring_core_0_17_5_ChaCha20_ctr32
#define LIMBS_add_mod ring_core_0_17_5_LIMBS_add_mod
#define LIMBS_are_even ring_core_0_17_5_LIMBS_are_even
#define LIMBS_are_zero ring_core_0_17_5_LIMBS_are_zero
#define LIMBS_equal ring_core_0_17_5_LIMBS_equal
#define LIMBS_equal_limb ring_core_0_17_5_LIMBS_equal_limb
#define LIMBS_less_than ring_core_0_17_5_LIMBS_less_than
#define LIMBS_less_than_limb ring_core_0_17_5_LIMBS_less_than_limb
#define LIMBS_reduce_once ring_core_0_17_5_LIMBS_reduce_once
#define LIMBS_select_512_32 ring_core_0_17_5_LIMBS_select_512_32
#define LIMBS_shl_mod ring_core_0_17_5_LIMBS_shl_mod
#define LIMBS_sub_mod ring_core_0_17_5_LIMBS_sub_mod
#define LIMBS_window5_split_window ring_core_0_17_5_LIMBS_window5_split_window
#define LIMBS_window5_unsplit_window ring_core_0_17_5_LIMBS_window5_unsplit_window
#define LIMB_shr ring_core_0_17_5_LIMB_shr
#define OPENSSL_armcap_P ring_core_0_17_5_OPENSSL_armcap_P
#define OPENSSL_cpuid_setup ring_core_0_17_5_OPENSSL_cpuid_setup
#define OPENSSL_ia32cap_P ring_core_0_17_5_OPENSSL_ia32cap_P
#define aes_hw_ctr32_encrypt_blocks ring_core_0_17_5_aes_hw_ctr32_encrypt_blocks
#define aes_hw_encrypt ring_core_0_17_5_aes_hw_encrypt
#define aes_hw_set_encrypt_key ring_core_0_17_5_aes_hw_set_encrypt_key
#define aes_nohw_ctr32_encrypt_blocks ring_core_0_17_5_aes_nohw_ctr32_encrypt_blocks
#define aes_nohw_encrypt ring_core_0_17_5_aes_nohw_encrypt
#define aes_nohw_set_encrypt_key ring_core_0_17_5_aes_nohw_set_encrypt_key
#define aesni_gcm_decrypt ring_core_0_17_5_aesni_gcm_decrypt
#define aesni_gcm_encrypt ring_core_0_17_5_aesni_gcm_encrypt
#define bn_from_montgomery_in_place ring_core_0_17_5_bn_from_montgomery_in_place
#define bn_gather5 ring_core_0_17_5_bn_gather5
#define bn_mul_mont ring_core_0_17_5_bn_mul_mont
#define bn_mul_mont_gather5 ring_core_0_17_5_bn_mul_mont_gather5
#define bn_neg_inv_mod_r_u64 ring_core_0_17_5_bn_neg_inv_mod_r_u64
#define bn_power5 ring_core_0_17_5_bn_power5
#define bn_scatter5 ring_core_0_17_5_bn_scatter5
#define bn_sqr8x_internal ring_core_0_17_5_bn_sqr8x_internal
#define bn_sqrx8x_internal ring_core_0_17_5_bn_sqrx8x_internal
#define bsaes_ctr32_encrypt_blocks ring_core_0_17_5_bsaes_ctr32_encrypt_blocks
#define bssl_constant_time_test_conditional_memcpy ring_core_0_17_5_bssl_constant_time_test_conditional_memcpy
#define bssl_constant_time_test_conditional_memxor ring_core_0_17_5_bssl_constant_time_test_conditional_memxor
#define bssl_constant_time_test_main ring_core_0_17_5_bssl_constant_time_test_main
#define chacha20_poly1305_open ring_core_0_17_5_chacha20_poly1305_open
#define chacha20_poly1305_seal ring_core_0_17_5_chacha20_poly1305_seal
#define fiat_curve25519_adx_mul ring_core_0_17_5_fiat_curve25519_adx_mul
#define fiat_curve25519_adx_square ring_core_0_17_5_fiat_curve25519_adx_square
#define gcm_ghash_avx ring_core_0_17_5_gcm_ghash_avx
#define gcm_ghash_clmul ring_core_0_17_5_gcm_ghash_clmul
#define gcm_ghash_neon ring_core_0_17_5_gcm_ghash_neon
#define gcm_gmult_clmul ring_core_0_17_5_gcm_gmult_clmul
#define gcm_gmult_neon ring_core_0_17_5_gcm_gmult_neon
#define gcm_init_avx ring_core_0_17_5_gcm_init_avx
#define gcm_init_clmul ring_core_0_17_5_gcm_init_clmul
#define gcm_init_neon ring_core_0_17_5_gcm_init_neon
#define k25519Precomp ring_core_0_17_5_k25519Precomp
#define limbs_mul_add_limb ring_core_0_17_5_limbs_mul_add_limb
#define little_endian_bytes_from_scalar ring_core_0_17_5_little_endian_bytes_from_scalar
#define ecp_nistz256_neg ring_core_0_17_5_ecp_nistz256_neg
#define ecp_nistz256_select_w5 ring_core_0_17_5_ecp_nistz256_select_w5
#define ecp_nistz256_select_w7 ring_core_0_17_5_ecp_nistz256_select_w7
#define p256_mul_mont ring_core_0_17_5_p256_mul_mont
#define p256_point_add ring_core_0_17_5_p256_point_add
#define p256_point_add_affine ring_core_0_17_5_p256_point_add_affine
#define p256_point_double ring_core_0_17_5_p256_point_double
#define p256_point_mul ring_core_0_17_5_p256_point_mul
#define p256_point_mul_base ring_core_0_17_5_p256_point_mul_base
#define p256_point_mul_base_vartime ring_core_0_17_5_p256_point_mul_base_vartime
#define p256_scalar_mul_mont ring_core_0_17_5_p256_scalar_mul_mont
#define p256_scalar_sqr_rep_mont ring_core_0_17_5_p256_scalar_sqr_rep_mont
#define p256_sqr_mont ring_core_0_17_5_p256_sqr_mont
#define p384_elem_div_by_2 ring_core_0_17_5_p384_elem_div_by_2
#define p384_elem_mul_mont ring_core_0_17_5_p384_elem_mul_mont
#define p384_elem_neg ring_core_0_17_5_p384_elem_neg
#define p384_elem_sub ring_core_0_17_5_p384_elem_sub
#define p384_point_add ring_core_0_17_5_p384_point_add
#define p384_point_double ring_core_0_17_5_p384_point_double
#define p384_point_mul ring_core_0_17_5_p384_point_mul
#define p384_scalar_mul_mont ring_core_0_17_5_p384_scalar_mul_mont
#define openssl_poly1305_neon2_addmulmod ring_core_0_17_5_openssl_poly1305_neon2_addmulmod
#define openssl_poly1305_neon2_blocks ring_core_0_17_5_openssl_poly1305_neon2_blocks
#define sha256_block_data_order ring_core_0_17_5_sha256_block_data_order
#define sha512_block_data_order ring_core_0_17_5_sha512_block_data_order
#define vpaes_ctr32_encrypt_blocks ring_core_0_17_5_vpaes_ctr32_encrypt_blocks
#define vpaes_encrypt ring_core_0_17_5_vpaes_encrypt
#define vpaes_encrypt_key_to_bsaes ring_core_0_17_5_vpaes_encrypt_key_to_bsaes
#define vpaes_set_encrypt_key ring_core_0_17_5_vpaes_set_encrypt_key
#define x25519_NEON ring_core_0_17_5_x25519_NEON
#define x25519_fe_invert ring_core_0_17_5_x25519_fe_invert
#define x25519_fe_isnegative ring_core_0_17_5_x25519_fe_isnegative
#define x25519_fe_mul_ttt ring_core_0_17_5_x25519_fe_mul_ttt
#define x25519_fe_neg ring_core_0_17_5_x25519_fe_neg
#define x25519_fe_tobytes ring_core_0_17_5_x25519_fe_tobytes
#define x25519_ge_double_scalarmult_vartime ring_core_0_17_5_x25519_ge_double_scalarmult_vartime
#define x25519_ge_frombytes_vartime ring_core_0_17_5_x25519_ge_frombytes_vartime
#define x25519_ge_scalarmult_base ring_core_0_17_5_x25519_ge_scalarmult_base
#define x25519_ge_scalarmult_base_adx ring_core_0_17_5_x25519_ge_scalarmult_base_adx
#define x25519_public_from_private_generic_masked ring_core_0_17_5_x25519_public_from_private_generic_masked
#define x25519_sc_mask ring_core_0_17_5_x25519_sc_mask
#define x25519_sc_muladd ring_core_0_17_5_x25519_sc_muladd
#define x25519_sc_reduce ring_core_0_17_5_x25519_sc_reduce
#define x25519_scalar_mult_adx ring_core_0_17_5_x25519_scalar_mult_adx
#define x25519_scalar_mult_generic_masked ring_core_0_17_5_x25519_scalar_mult_generic_masked

#endif
