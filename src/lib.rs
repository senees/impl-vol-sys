/*
 * MIT License
 *
 * Copyright (c) 2022 seenees
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

extern crate cxx;

use libc::c_double;

#[cxx::bridge]
mod ffi {}

extern "C" {
  fn implied_volatility_from_a_transformed_rational_guess(
    price: c_double,
    f: c_double,
    k: c_double,
    t: c_double,
    q: c_double,
  ) -> c_double;
}

pub fn iv_implied_volatility_from_a_transformed_rational_guess(
  price: f64,
  forward: f64,
  strike: f64,
  time: f64,
  quote: f64,
) -> f64 {
  unsafe {
    implied_volatility_from_a_transformed_rational_guess(
      price, forward, strike, time, quote,
    )
  }
}
