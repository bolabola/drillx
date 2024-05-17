/* Copyright (c) 2020 tevador <tevador@gmail.com> */
/* See LICENSE for licensing information */

#include "siphash_rng.h"

__device__ void hashx_siphash_rng_init(siphash_rng* gen, const siphash_state* state) {
	gen->keys = *state;
	gen->counter = 0;
	gen->count8 = 0;
	gen->count32 = 0;
}

__device__ uint8_t hashx_siphash_rng_u8(siphash_rng* gen) {
	if (gen->count8 == 0) {
		gen->buffer8 = hashx_siphash13_ctr(gen->counter, &gen->keys);
		gen->counter++;
		gen->count8 = sizeof(gen->buffer8);
	}
	gen->count8--;
	return gen->buffer8 >> (gen->count8 * 8);
}

__device__ uint32_t hashx_siphash_rng_u32(siphash_rng* gen) {
	if (gen->count32 == 0) {
		gen->buffer32 = hashx_siphash13_ctr(gen->counter, &gen->keys);
		gen->counter++;
		gen->count32 = sizeof(gen->buffer32) / sizeof(uint32_t);
	}
	gen->count32--;
	return gen->buffer32 >> (gen->count32 * 32);
}
