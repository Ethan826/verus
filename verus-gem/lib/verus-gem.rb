require "ffi"

module Verus
  extend FFI::Library

  ffi_lib "./bin/libverus_ruby_wrappers." + FFI::Platform::LIBSUFFIX

  attach_function :validate_email, :email, [:string], :bool
  attach_function :validate_date, :date, [:string], :bool
end
