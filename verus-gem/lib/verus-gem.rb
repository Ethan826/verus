require "ffi"

module Verus
  extend FFI::Library

  ffi_lib "../verus-ruby-wrappers/target/release/libverus_ruby_wrappers." + FFI::Platform::LIBSUFFIX

  attach_function :validate_email, [:string], :bool
  attach_function :validate_date, [:string], :bool
end