/*
 * Pipeline module
 *
 * This is an initial scaffold for a future encoding/obfuscation pipeline
 * where multiple processing stages can be chained together.
 *
 * You can extend it with concrete stages that implement `Stage` and then
 * compose them with `Pipeline::with_stage(...)`.
 */
pub mod encode;
pub mod parser;

