require "ffi"

module Verus
  extend FFI::Library

  ffi_lib "../verus-gem/bin/libverus_ruby_wrappers.so"

  attach_function :validate_email, :email, [:string], :bool
  attach_function :validate_date, :date, [:string], :bool
end
