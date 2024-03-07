import { setTo } from '@iron-e/scabbard';
import { CARGO_HACK_VERSION } from '@iron-e/scabbard/rust/scope/client';

setTo('0.6.20', CARGO_HACK_VERSION);

/**
 * Default arguments for cargo hack.
 * HACK: `--at-least-one-of` requires at least two features, so for now `'markdown,markdown'` will have to do.
 */
export const CARGO_HACK_ARGS = ['--feature-powerset', '--at-least-one-of', 'markdown,markdown'];
