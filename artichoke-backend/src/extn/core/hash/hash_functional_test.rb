# test_hash_functional.rb
# frozen_string_literal: true

##
# The main function that calls all tests in order.
# You can reorder or selectively comment out as needed.
def spec
  test_hash_new
  test_hash_store_and_access
  test_hash_default
  test_hash_fetch
  test_hash_delete
  test_hash_clear
  test_hash_empty_and_size
  test_hash_keys_values
  test_hash_include_and_has_key
  test_hash_has_value
  test_hash_shift
  test_hash_merge
  test_hash_replace
  test_hash_dup_clone
  test_hash_rehash
  test_hash_initialize_copy

  test_hash_merge_sad_path
  test_hash_merge_block_edge_cases
  expect_failure_if_artichoke(RuntimeError, 'Expected TypeError assigning non-callable default_proc, but none was raised!') do
    # Pending patching of the bug in https://github.com/mruby/mruby/issues/6483
    test_hash_default_proc_sad_path
  end
  expect_failure_if_artichoke(RuntimeError, 'Expected TypeError for 0-arg default_proc, but no error was raised!') do
    # Pending patching of the bug in https://github.com/mruby/mruby/issues/6484
    test_hash_default_proc_wrong_arity
  end

  test_hash_fetch_sad_path
  expect_failure_if_artichoke(RuntimeError, 'Unexpected error message: "Expected error from unhashable key"') do
    # Pending patching of the bug in https://github.com/mruby/mruby/issues/6486
    test_hash_key_type_errors
  end
  test_hash_delete_sad_path
  test_hash_store_edge_cases
  test_hash_block_modification

  test_hash_merge_bang_no_args
  test_hash_merge_bang_multi_args_no_block
  test_hash_merge_bang_multi_args_with_block
  test_hash_merge_bang_multi_args_type_check
  test_hash_merge_bang_multi_args_self_referential

  test_hash_merge_coerce_success
  test_hash_merge_coerce_with_block
  test_hash_merge_coerce_non_hash
  test_hash_merge_coerce_nil
  test_hash_merge_coerce_raises
  test_hash_merge_non_coercible

  test_hash_merge_bang_coerce_success
  test_hash_merge_bang_coerce_with_block
  test_hash_merge_bang_coerce_non_hash
  test_hash_merge_bang_coerce_nil
  test_hash_merge_bang_coerce_raises
  test_hash_merge_bang_non_coercible
  test_hash_merge_bang_multi_arg_coercion_with_collisions

  test_hash_replace_implicit_conversion_success
  test_hash_replace_implicit_conversion_non_hash
  test_hash_replace_implicit_conversion_nil
  test_hash_replace_implicit_conversion_raises
  test_hash_replace_implicit_conversion_non_coercible

  test_hash_replace_basic
  test_hash_replace_with_default_proc
  test_hash_replace_with_default_value
  test_hash_replace_same_object

  test_hash_frozen_error_on_mutating_methods
end

##
# Runs the given block. Behavior depends on whether we're in Artichoke:
#
# - If `RUBY_ENGINE.start_with?('artichoke')`:
#   * Expect a specific error class and message.
#   * If no error is raised, fail.
#   * If an error is raised that doesn't match `error_class` or `error_message`, fail.
#   * Otherwise pass.
#
# - If not Artichoke:
#   * Expect no error.
#   * If an error is raised, re-raise it.
#
def expect_failure_if_artichoke(error_class, error_message)
  is_artichoke = RUBY_ENGINE.start_with?('artichoke')

  begin
    yield
    # If we get here, no error was raised
    if is_artichoke
      raise "Expected #{error_class} with message #{error_message.inspect} on Artichoke, but no error was raised!"
    end
  rescue error_class => e
    # If we catch the expected error type
    raise "Did not expect any error on non-Artichoke! (Got #{error_class}: #{e.message.inspect})" unless is_artichoke
    unless e.message == error_message
      raise "Unexpected error message: #{e.message.inspect}\nExpected: #{error_message.inspect}"
    end
    # If message matches, we pass
  end
end

##
# Test suite for a Ruby `Hash` implemented in Rust (spinoso-hash).
# Each API or family of APIs is tested in its own method.
#
# Usage:
#
#   # If running inside Artichoke or another environment that
#   # uses spinoso-hash as its `Hash`:
#   require_relative 'test_hash_functional'
#   spec
#

def test_hash_new
  # Check we can create an empty hash
  h = {}
  raise 'Expected empty hash' unless h.empty?
  raise 'Expected hash size 0' unless h.size == 0 # rubocop:disable Style/ZeroLengthPredicate,Style/NumericPredicate

  # Check we can create a hash with a default value
  hdef = Hash.new(99)
  raise 'Expected default of 99' unless hdef[1] == 99
  raise 'Expected empty actual map' unless hdef.size == 0 # rubocop:disable Style/ZeroLengthPredicate,Style/NumericPredicate

  # Check we can create a hash with a default proc
  hproc = Hash.new { |ht, k| ht[k] = "computed-#{k}" }
  raise 'Expected dynamic default' unless hproc[:foo] == 'computed-foo'
  raise 'Expected :foo to now exist' unless hproc.has_key?(:foo) # rubocop:disable Style/PreferredHashMethods
  raise 'Expected :foo to now exist' unless hproc.key?(:foo)
  raise 'Expected size 1' unless hproc.size == 1
end

def test_hash_store_and_access
  # store == []=
  h = {}
  h[:a] = 10
  raise 'Expected 10' unless h[:a] == 10

  h.store(:b, 20)
  raise 'Expected 20' unless h[:b] == 20
  raise 'Expected size 2' unless h.size == 2

  # Overwrite
  h[:a] = 999
  raise 'Expected 999' unless h[:a] == 999
  raise 'Expected size still 2' unless h.size == 2

  # Access nonexistent
  raise 'Expected nil' unless h[:unknown].nil?
end

def test_hash_default
  # default and default=
  h = {}
  raise 'Initially no default' unless h.default.nil?

  h.default = :foo
  raise 'Expected :foo' unless h.default == :foo
  raise 'Expected :foo' unless h[:any_missing_key] == :foo

  # default_proc and default_proc=
  h2 = {}
  raise 'Expected nil' unless h2.default_proc.nil?

  h2.default_proc = proc { |hh, k| hh[k] = k.to_s.upcase }
  val = h2[:abc]
  raise "Expected 'ABC'" unless val == 'ABC'
  raise "Expected h2 to store 'ABC'" unless h2[:abc] == 'ABC'
  raise 'Expected size 1' unless h2.size == 1

  # Removing default
  h2.default_proc = nil
  raise 'Expected no default proc' unless h2.default_proc.nil?
  raise 'Expected nil' unless h2[:missing].nil?
end

def test_hash_fetch
  h = { x: 10, y: 20 }

  # fetch
  raise 'Expected 10' unless h.fetch(:x) == 10

  begin
    h.fetch(:z)
    raise 'Expected KeyError'
  rescue KeyError
    # OK
  end

  # fetch with default value
  val = h.fetch(:z, 999)
  raise 'Expected 999' unless val == 999

  # fetch with block
  val = h.fetch(:nope) { |missing| "computed-#{missing}" }
  raise "Expected 'computed-nope'" unless val == 'computed-nope'
end

def test_hash_delete
  h = { a: 1, b: 2, c: 3 }
  val = h.delete(:b)
  raise 'Expected 2' unless val == 2
  raise 'Expected no :b' if h.key?(:b)

  missing = h.delete(:z)
  raise 'Expected nil' unless missing.nil?
end

def test_hash_clear
  h = { a: 1, b: 2 }
  h.clear
  raise 'Expected empty' unless h.empty?
  raise 'Expected size 0' unless h.size == 0 # rubocop:disable Style/ZeroLengthPredicate,Style/NumericPredicate
end

def test_hash_empty_and_size
  h = {}
  raise 'Expected empty' unless h.empty?

  h[:x] = 42
  raise 'Expected not empty' if h.empty?
  raise 'Expected size 1' unless h.size == 1
  raise 'Expected length 1' unless h.length == 1
end

def test_hash_keys_values
  h = { a: 10, b: 20, c: 30 }
  ks = h.keys
  vs = h.values
  raise 'Keys mismatch' unless ks.sort == %i[a b c]
  raise 'Values mismatch' unless vs.sort == [10, 20, 30]
end

def test_hash_include_and_has_key
  h = { a: 10 }
  # has_key?, key?, include?, member?
  raise 'Expected has_key?(:a)' unless h.has_key?(:a)  # rubocop:disable Style/PreferredHashMethods
  raise 'Expected key?(:a)' unless h.key?(:a)
  raise 'Expected include?(:a)' unless h.include?(:a)
  raise 'Expected member?(:a)' unless h.member?(:a)

  raise 'Expected not has_key?(:b)' if h.has_key?(:b)  # rubocop:disable Style/PreferredHashMethods
  raise 'Expected not key?(:b)' if h.key?(:b)
  raise 'Expected not include?(:b)' if h.include?(:b)
  raise 'Expected not member?(:b)' if h.member?(:b)
end

def test_hash_has_value
  h = { a: 10, b: 20 }
  # has_value?, value?
  raise 'Expected has_value?(10)' unless h.has_value?(10) # rubocop:disable Style/PreferredHashMethods
  raise 'Expected value?(20)' unless h.value?(20)
  raise 'Expected no 99' if h.value?(99)
end

def test_hash_shift
  h = { a: 1, b: 2, c: 3 }
  pair = h.shift
  raise 'Expected [:a, 1]' unless pair == [:a, 1]
  raise 'Expected size 2' unless h.size == 2

  pair2 = h.shift
  raise 'Expected one more' unless pair2 == [:b, 2]

  pair3 = h.shift
  raise 'Expected [:c, 3]' unless pair3 == [:c, 3]

  pair4 = h.shift
  raise 'Expected nil' unless pair4.nil?
  raise 'Expected size 0' unless h.size == 0 # rubocop:disable Style/ZeroLengthPredicate,Style/NumericPredicate
end

def test_hash_merge
  h1 = { a: 1, b: 2 }
  h2 = { b: 20, c: 30 }
  # Merge without a block
  h1.merge!(h2)
  # => now h1 = { a: 1, b: 20, c: 30 }
  raise 'Expected 1' unless h1[:a] == 1
  raise 'Expected 20' unless h1[:b] == 20
  raise 'Expected 30' unless h1[:c] == 30

  # Merge with a block
  h3 = { x: 100, y: 200 }
  h4 = { y: 999, z: 111 }
  out = h3.merge(h4) { |_key, old_val, new_val| old_val + new_val }
  # => x=100, y=(200+999=1199), z=111
  raise "Expected { x: 100, y: 1199, z: 111 } got #{out}" unless out[:x] == 100 && out[:y] == 1199 && out[:z] == 111
end

def test_hash_replace
  h1 = { a: 1, b: 2 }
  h2 = { x: 10 }
  h1.replace(h2)
  raise 'Expected size 1' unless h1.size == 1
  raise 'Expected 10' unless h1[:x] == 10
  raise 'Expected no :a' if h1.key?(:a)
end

def test_hash_dup_clone
  h = { a: 1, b: 2 }
  duped = h.dup
  raise 'Expected same content' unless duped[:a] == 1 && duped[:b] == 2

  duped[:a] = 99
  # original unaffected
  raise 'Original changed' if h[:a] == 99

  # Some rubies treat clone similarly:
  cloned = h.clone
  raise 'Expected same content' unless cloned[:b] == 2

  cloned[:b] = 123
  raise 'Original changed' if h[:b] == 123
end

def test_hash_rehash
  # Simple rehash test. Typically, you'd do something like store a key that
  # modifies its #hash. For a functional test, let's just call rehash and see
  # that it doesn't blow up or reorder incorrectly.
  h = {}
  h['foo'] = 1
  h['bar'] = 2
  h.rehash # no error
  raise 'Expected 2 elements' unless h.size == 2
  raise 'Expected 1' unless h['foo'] == 1
  raise 'Expected 2' unless h['bar'] == 2
end

def test_hash_initialize_copy
  # #initialize_copy is usually an internal callback from dup/clone.
  # We check that copying from one hash to another yields the same pairs.
  orig = { a: 1, b: 2 }
  copy = {}
  copy.send(:initialize_copy, orig)
  raise 'Expected 2 elements' unless copy.size == 2
  raise 'Expected same pairs' unless copy[:a] == 1 && copy[:b] == 2
end

def test_hash_merge_sad_path
  # Merge with non-Hash
  begin
    h = { a: 1 }
    h.merge(123)
    raise 'Expected TypeError for merging non-Hash, but none was raised!'
  rescue TypeError
    # pass
  end

  # Merge with nil
  begin
    h.merge(nil)
    raise 'Expected TypeError for merging nil, but none was raised!'
  rescue TypeError
    # pass
  end

  # Merge with self (should be fine if implemented carefully)
  h2 = { b: 2 }
  out = h2.merge(h2) { |_k, old_val, new_val| old_val + new_val }
  # This is an odd scenario, but Ruby does handle it gracefully.
  # b => 2 + 2 = 4
  raise "Expected { b: 4 }, got #{out}" unless out == { b: 4 }
end

def test_hash_merge_block_edge_cases
  # Return unusual values from the block
  h = { a: 10, b: 20 }
  other = { b: 999, c: 777 }
  out = h.merge(other) do |_k, old_val, new_val|
    # Arbitrary or weird return:
    [old_val, new_val, :merged]
  end
  # => a => 10, b => [20, 999, :merged], c => 777
  raise 'Expected a => 10' unless out[:a] == 10
  raise 'Expected b => [20, 999, :merged]' unless out[:b] == [20, 999, :merged]
  raise 'Expected c => 777' unless out[:c] == 777
end

def test_hash_default_proc_sad_path
  h = {}
  # Attempt to set the default proc to a non-callable
  begin
    h.default_proc = 123
    raise 'Expected TypeError assigning non-callable default_proc, but none was raised!'
  rescue TypeError => e
    unless e.message == 'wrong default_proc type Integer (expected Proc)'
      raise "got wrong exception message: #{e.message}"
    end
    # pass
  end

  # Attempt to retrieve default proc from a non-proc-based default
  h.default = :some_default
  return unless h.default_proc

  raise 'Expected default_proc to be nil when default is a simple value'
end

def test_hash_default_proc_wrong_arity
  h = {}

  # 0-argument lambda => "default_proc takes two arguments (2 for 0)"
  begin
    h.default_proc = -> { 1 }
    raise 'Expected TypeError for 0-arg default_proc, but no error was raised!'
  rescue TypeError => e
    unless e.message == 'default_proc takes two arguments (2 for 0)'
      raise "Expected \"default_proc takes two arguments (2 for 0)\", got #{e.message.inspect}"
    end
  end

  # 1-argument lambda => "default_proc takes two arguments (2 for 1)"
  begin
    h.default_proc = ->(_a) { 1 }
    raise 'Expected TypeError for 1-arg default_proc, but no error was raised!'
  rescue TypeError => e
    unless e.message == 'default_proc takes two arguments (2 for 1)'
      raise "Expected \"default_proc takes two arguments (2 for 1)\", got #{e.message.inspect}"
    end
  end

  # 4-argument lambda => "default_proc takes two arguments (2 for 4)"
  begin
    h.default_proc = ->(_a, _b, _c, _d) { 1 }
    raise 'Expected TypeError for 4-arg default_proc, but no error was raised!'
  rescue TypeError => e
    unless e.message == 'default_proc takes two arguments (2 for 4)'
      raise "Expected \"default_proc takes two arguments (2 for 4)\", got #{e.message.inspect}"
    end
  end
end

def test_hash_fetch_sad_path
  # fetch with no block, no default, missing key => KeyError
  h = { x: 1 }
  begin
    h.fetch(:missing)
    raise 'Expected KeyError for missing key with no default, but none was raised!'
  rescue KeyError
    # pass
  end

  # fetch with a block that raises an error
  begin
    h.fetch(:not_there) do |_k|
      raise 'some inner error'
    end
    raise 'Expected RuntimeError in fetch block, but none was raised!'
  rescue RuntimeError => e
    raise "Expected 'some inner error', got #{e.message.inspect}" unless e.message == 'some inner error'
  end
end

module Artichoke
  module FunctionalTests
    class Unhashable
      def hash
        raise 'Unhashable#hash called'
      end

      def eql?(other)
        other.is_a?(Unhashable)
      end
    end
  end
end

def test_hash_key_type_errors
  h = {}
  begin
    h[Artichoke::FunctionalTests::Unhashable.new] = 1
    raise 'Expected error from unhashable key'
  rescue RuntimeError => e
    raise "Unexpected error message: #{e.message.inspect}" unless e.message == 'Unhashable#hash called'
  end
end

def test_hash_delete_sad_path
  h = { a: 1 }
  # Passing no args to delete => ArgumentError in real Ruby
  begin
    h.delete
    raise 'Expected ArgumentError for missing argument to Hash#delete'
  rescue ArgumentError
    # pass
  end

  # Passing multiple args to delete => ArgumentError in real Ruby
  begin
    h.delete(:a, :b)
    raise 'Expected ArgumentError for multiple arguments to Hash#delete'
  rescue ArgumentError
    # pass
  end
end

def test_hash_store_edge_cases
  # Storing nil, false, etc.
  h = {}
  h[nil] = 'value for nil'
  raise "Expected 'value for nil' for h[nil]" unless h[nil] == 'value for nil'

  h[false] = :false_val
  raise 'Expected :false_val for h[false]' unless h[false] == :false_val

  # Overwriting the same key with different value types
  h[:x] = 42
  h[:x] = 'forty-two'
  raise "Expected 'forty-two' for h[:x]" unless h[:x] == 'forty-two'
end

def test_hash_block_modification
  # Some enumerators in Ruby raise an error if the hash is modified
  # inside the block, e.g. .each, .each_key.
  # We'll do a small check with .each_key
  h = { a: 1, b: 2, c: 3 }
  begin
    h.each_key do |k|
      h.delete(k) if k == :b
    end
    raise 'Expected RuntimeError or similar (hash modified during iteration).'
  rescue RuntimeError
    # This is standard MRI behavior, might differ in your environment
    # If your environment doesn't raise, you can remove or skip this test.
  end
end

def test_hash_merge_bang_no_args
  h = { x: 1, y: 2 }
  orig = h.object_id

  # No arguments, no block
  returned = h.merge!
  raise 'Expected merge! with no args to return self' if returned.object_id != orig
  raise "Expected no changes to h, got #{returned.inspect}" unless returned == { x: 1, y: 2 }

  # No arguments, with a block => block is ignored
  called_block = false
  returned2 = h.merge! do |_key, _old_val, _new_val|
    called_block = true
    :should_not_happen
  end
  raise 'Block was called even though no merges are happening!' if called_block
  return if returned2 == h && returned2.object_id == orig

  raise 'Expected still the same object, got a different one'
end

def test_hash_merge_bang_multi_args_no_block
  base = { foo: 0, bar: 1, baz: 2 }
  h1   = { bat: 3, bar: 4 }    # bar => 4
  h2   = { bam: 5, bat: 6 }    # bat => 6

  # Merge them left to right => h1 merges first, then h2
  returned = base.merge!(h1, h2)
  raise 'Expected #merge! to return self, not a new object' unless returned.equal?(base)

  # After merging h1 => :bar => 4, :bat => 3
  # After merging h2 => :bat => 6, :bam => 5
  expected = { foo: 0, bar: 4, baz: 2, bat: 6, bam: 5 }
  return if base == expected

  raise "Expected #{expected.inspect}, got #{base.inspect}"
end

def test_hash_merge_bang_multi_args_with_block
  base = { foo: 0, bar: 1, baz: 2 }
  h1   = { bat: 3, bar: 4 }
  h2   = { bam: 5, bat: 6 }

  # If there's a duplicate, the block is called with (key, old_val, new_val).
  returned = base.merge!(h1, h2) do |_key, old_val, new_val|
    old_val + new_val
  end

  raise 'Expected #merge! to return self' unless returned.equal?(base)

  # Merged left to right:
  # 1) Merge h1 => bar => old(1)+new(4)=5, bat => 3
  # 2) Merge h2 => bat => old(3)+new(6)=9, bam => 5
  # So final shape:
  # foo => 0 (unchanged),
  # bar => 5,
  # baz => 2,
  # bat => 9,
  # bam => 5
  expected = { foo: 0, bar: 5, baz: 2, bat: 9, bam: 5 }
  return if base == expected

  raise "Expected #{expected.inspect}, got #{base.inspect}"
end

def test_hash_merge_bang_multi_args_type_check
  base = { a: 1 }
  # second arg is not a Hash
  begin
    base.merge!({ b: 2 }, 123)
    raise 'Expected TypeError merging a non-hash argument, but none was raised!'
  rescue TypeError => e
    # pass: "Hash required (Integer given)"
    unless e.message == 'no implicit conversion of Integer into Hash'
      raise "Unexpected error message: #{e.message.inspect}"
    end
  end
end

def test_hash_merge_bang_multi_args_self_referential
  h = { x: 1, y: 2 }
  # Merge h with itself multiple times
  # If done left->right, each pass might double the value if there's a block
  h.merge!(h, h) do |_key, old_val, new_val|
    old_val + new_val
  end
  # 1) first merge => x => 1+1=2, y => 2+2=4
  # 2) second merge => x => 2+2=4, y => 4+4=8
  expected = { x: 4, y: 8 }
  return if h == expected

  raise "Expected self merges to produce #{expected.inspect}, got #{h.inspect}"
end

module Artichoke
  module FunctionalTests
    class HashCoercible
      def to_hash
        { coerced_key: 'coerced_value' }
      end
    end

    class HashCoercibleWithBlock
      def to_hash
        { duplicated: 1 }
      end
    end

    class HashCoercibleNonHash
      def to_hash
        123
      end
    end

    class HashCoercibleNil
      def to_hash
        nil
      end
    end

    class HashCoercibleRaise
      def to_hash
        raise 'Intentional error in to_hash'
      end
    end

    class HashNonCoercible # rubocop:disable Lint/EmptyClass
      # no #to_hash method
    end
  end
end

def test_hash_merge_coerce_success
  base = { existing: 42 }
  obj = Artichoke::FunctionalTests::HashCoercible.new

  out = base.merge(obj)
  # We expect base to remain unchanged, out is a new hash
  unless out[:existing] == 42 && out[:coerced_key] == 'coerced_value'
    raise "Expected merged result to include {existing: 42, coerced_key: 'coerced_value'}, got #{out.inspect}"
  end
  return if base.size == 1 && base[:existing] == 42

  raise "Expected original hash unchanged, got #{base.inspect}"
end

def test_hash_merge_coerce_with_block
  base = { duplicated: 10, unique: 99 }
  obj = Artichoke::FunctionalTests::HashCoercibleWithBlock.new
  # => obj.to_hash => { duplicated: 1 }
  # Using a block for the duplicate key

  out = base.merge(obj) do |_key, old_val, new_val|
    old_val + new_val
  end
  # => duplicated => 10+1=11, plus unique => 99
  expected = { duplicated: 11, unique: 99 }
  raise "Expected #{expected.inspect}, got #{out.inspect}" unless out == expected
  return if base == { duplicated: 10, unique: 99 }

  raise "Expected original hash unchanged, got #{base.inspect}"
end

def test_hash_merge_coerce_non_hash
  base = { a: 1 }
  obj = Artichoke::FunctionalTests::HashCoercibleNonHash.new
  # => to_hash returns 123, not a Hash
  begin
    base.merge(obj)
    raise "Expected TypeError merging a 'HashCoercibleNonHash', but none raised!"
  rescue TypeError => e
    unless e.message == "can't convert Artichoke::FunctionalTests::HashCoercibleNonHash to Hash (Artichoke::FunctionalTests::HashCoercibleNonHash#to_hash gives Integer)"
      raise "Unexpected error message: #{e.message.inspect}"
    end
  end
end

def test_hash_merge_coerce_nil
  base = { a: 1 }
  obj = Artichoke::FunctionalTests::HashCoercibleNil.new
  begin
    base.merge(obj)
    raise 'Expected TypeError merging nil-coerce, but none raised!'
  rescue TypeError => e
    unless e.message == "can't convert Artichoke::FunctionalTests::HashCoercibleNil to Hash (Artichoke::FunctionalTests::HashCoercibleNil#to_hash gives NilClass)"
      raise "Unexpected error message: #{e.message.inspect}"
    end
  end
end

def test_hash_merge_coerce_raises
  base = { a: 1 }
  obj = Artichoke::FunctionalTests::HashCoercibleRaise.new
  begin
    base.merge(obj)
    raise 'Expected RuntimeError from to_hash, but none raised!'
  rescue RuntimeError => e
    raise "Unexpected message: #{e.message.inspect}" unless e.message == 'Intentional error in to_hash'
  end
end

def test_hash_merge_non_coercible
  base = { a: 1 }
  obj = Artichoke::FunctionalTests::HashNonCoercible.new
  begin
    base.merge(obj)
    raise "Expected TypeError for an object that doesn't respond to to_hash, but none raised!"
  rescue TypeError => e
    unless e.message == 'no implicit conversion of Artichoke::FunctionalTests::HashNonCoercible into Hash'
      raise "Unexpected error message: #{e.message.inspect}"
    end
  end
end

def test_hash_merge_bang_coerce_success
  base = { existing: 42 }
  obj = Artichoke::FunctionalTests::HashCoercible.new
  orig_id = base.object_id

  returned = base.merge!(obj)
  raise 'Expected merge! to return self, not a new object' unless returned.object_id == orig_id

  # The coerced hash => {coerced_key: "coerced_value"}
  # Merged in place
  return if base[:existing] == 42 && base[:coerced_key] == 'coerced_value'

  raise "Expected base to be updated with coerced_key, got #{base.inspect}"
end

def test_hash_merge_bang_coerce_with_block
  base = { duplicated: 10, other: 5 }
  obj = Artichoke::FunctionalTests::HashCoercibleWithBlock.new
  # => to_hash => { duplicated: 1 }

  base_id = base.object_id
  returned = base.merge!(obj) do |_k, old_val, new_val|
    old_val + new_val
  end
  raise 'Expected merge! to return self' unless returned.equal?(base)
  raise 'Expected same object_id' unless base.object_id == base_id

  # Merge in place => duplicated => 10+1=11, other => 5
  expected = { duplicated: 11, other: 5 }
  return if base == expected

  raise "Expected #{expected.inspect}, got #{base.inspect}"
end

def test_hash_merge_bang_coerce_non_hash
  base = { a: 1 }
  obj = Artichoke::FunctionalTests::HashCoercibleNonHash.new
  begin
    base.merge!(obj)
    raise 'Expected TypeError merging a non-hash from #to_hash, but none was raised!'
  rescue TypeError => e
    unless e.message == "can't convert Artichoke::FunctionalTests::HashCoercibleNonHash to Hash (Artichoke::FunctionalTests::HashCoercibleNonHash#to_hash gives Integer)"
      raise "Unexpected error message: #{e.message.inspect}"
    end
  end
end

def test_hash_merge_bang_coerce_nil
  base = { a: 1 }
  obj = Artichoke::FunctionalTests::HashCoercibleNil.new
  begin
    base.merge!(obj)
    raise 'Expected TypeError merging a nil from #to_hash, but none was raised!'
  rescue TypeError => e
    unless e.message == "can't convert Artichoke::FunctionalTests::HashCoercibleNil to Hash (Artichoke::FunctionalTests::HashCoercibleNil#to_hash gives NilClass)"
      raise "Unexpected error message: #{e.message.inspect}"
    end
  end
end

def test_hash_merge_bang_coerce_raises
  base = { a: 1 }
  obj = Artichoke::FunctionalTests::HashCoercibleRaise.new
  begin
    base.merge!(obj)
    raise 'Expected RuntimeError from #to_hash, but none was raised!'
  rescue RuntimeError => e
    raise "Unexpected message: #{e.message.inspect}" unless e.message == 'Intentional error in to_hash'
  end
end

def test_hash_merge_bang_non_coercible
  base = { a: 1 }
  obj = Artichoke::FunctionalTests::HashNonCoercible.new
  begin
    base.merge!(obj)
    raise "Expected TypeError for object that doesn't respond to #to_hash, but none raised!"
  rescue TypeError => e
    unless e.message == 'no implicit conversion of Artichoke::FunctionalTests::HashNonCoercible into Hash'
      raise "Unexpected error message: #{e.message.inspect}"
    end
  end
end

def test_hash_merge_bang_multi_arg_coercion_with_collisions
  base = { x: 1, duplicated: 10 }
  c1   = Artichoke::FunctionalTests::HashCoercible.new           # => { coerced_key: "coerced_value" }
  c2   = Artichoke::FunctionalTests::HashCoercibleWithBlock.new  # => { duplicated: 1 }

  base.merge!(c1, c2) do |_key, old_val, new_val|
    "#{old_val}+#{new_val}"
  end
  # After c1 => base => { x:1, duplicated:10, coerced_key:"coerced_value" }
  # c1 has no duplicate keys => no block call
  # After c2 => 'duplicated' => old=10, new=1 => block => "10+1"
  # So final => { x:1, duplicated:"10+1", coerced_key:"coerced_value" }

  expected = { x: 1, duplicated: '10+1', coerced_key: 'coerced_value' }
  return if base == expected

  raise "Expected multi-arg merge with collisions, got #{base.inspect}"
end

def test_hash_replace_implicit_conversion_success
  base = { old_key: 99 }
  obj  = Artichoke::FunctionalTests::HashCoercible.new
  returned = base.replace(obj)
  raise 'Expected #replace to return self, not a new object' unless returned.equal?(base)
  return if base == { coerced_key: 'coerced_value' }

  raise "Expected base to be replaced with coerced hash, got #{base.inspect}"
end

def test_hash_replace_implicit_conversion_non_hash
  base = { a: 1 }
  obj  = Artichoke::FunctionalTests::HashCoercibleNonHash.new
  begin
    base.replace(obj)
    raise 'Expected TypeError from #to_hash => 123, but none was raised!'
  rescue TypeError
    # pass
  end
end

def test_hash_replace_implicit_conversion_nil
  base = { a: 1 }
  obj  = Artichoke::FunctionalTests::HashCoercibleNil.new
  begin
    base.replace(obj)
    raise 'Expected TypeError from #to_hash => nil, but none was raised!'
  rescue TypeError
    # pass
  end
end

def test_hash_replace_implicit_conversion_raises
  base = { a: 1 }
  obj  = Artichoke::FunctionalTests::HashCoercibleRaise.new
  begin
    base.replace(obj)
    raise 'Expected RuntimeError from #to_hash, but none was raised!'
  rescue RuntimeError => e
    raise "Unexpected message: #{e.message.inspect}" unless e.message == 'Intentional error in to_hash'
  end
end

def test_hash_replace_implicit_conversion_non_coercible
  base = { a: 1 }
  obj  = Artichoke::FunctionalTests::HashNonCoercible.new
  begin
    base.replace(obj)
    raise 'Expected TypeError for an object without #to_hash, but none was raised!'
  rescue TypeError
    # pass
  end
end

def test_hash_replace_basic
  h = { a: 1, b: 2, default: :old_default }
  h.default = :old_default
  # Let's define a custom default_proc to ensure we overwrite it
  # if you prefer a default_proc test. We'll do a separate test below.

  other = { x: 100, y: 200 }
  other.default = :new_default

  returned = h.replace(other)
  raise 'Expected #replace to return self, got a different object!' unless returned.equal?(h)

  # The keys should be replaced
  raise "Expected h to have [:x, :y] after replace, got #{h.keys.inspect}" unless h.keys.sort == %i[x y]
  raise 'Expected h[:x] == 100, h[:y] == 200' unless h[:x] == 100 && h[:y] == 200

  # Default is replaced
  return if h.default == :new_default

  raise "Expected default to be replaced with :new_default, got #{h.default.inspect}"
end

def test_hash_replace_with_default_proc
  h = { a: 1 }
  h.default = :old_def

  other = { b: 2 }
  other.default_proc = proc { |_hash, key| "computed-#{key}" }

  h.replace(other)
  raise 'Expected h to have only key :b after replace' unless h.keys == [:b]
  raise 'Expected default_proc to be copied, but h.default_proc is nil' unless h.default_proc

  # Call the default_proc to verify it
  val = h[:missing]
  return if val == 'computed-missing'

  raise "Expected default_proc to compute 'computed-missing', got #{val.inspect}"
end

def test_hash_replace_with_default_value
  h = {}
  h.default_proc = proc { |_h, _k| raise 'Should not happen' }

  other = {}
  other.default = :simple_def

  h.replace(other)
  raise "Expected h.default_proc to be cleared (no block), but got #{h.default_proc.inspect}" unless h.default_proc.nil?
  return if h.default == :simple_def

  raise "Expected h.default to be :simple_def, got #{h.default.inspect}"
end

def test_hash_replace_same_object
  h = { x: 1, y: 2 }
  ret = h.replace(h) # from MRI doc: replacing with the same object is effectively no-op
  raise 'Expected #replace to return self' unless ret.equal?(h)
  # No changes
  return if h == { x: 1, y: 2 }

  raise "Expected no change if replacing with itself, but got #{h.inspect}"
end

def test_hash_frozen_error_on_mutating_methods
  # Prepare a hash and freeze it
  h = { a: 1 }
  h.freeze

  # Simple helper to DRY up the rescue logic
  expect_frozen_error = lambda do |label, &blk|
    blk.call
    raise "Expected FrozenError calling: #{label}, but no error was raised!"
  rescue FrozenError
    # pass, this is expected
  end

  # 1. Element assignment: `h[:b] = 2`
  expect_frozen_error.call('[]= store') do
    h[:b] = 2
  end

  # 2. Delete a key
  expect_frozen_error.call('delete(:a)') do
    h.delete(:a)
  end

  # 3. Shift
  expect_frozen_error.call('shift') do
    h.shift
  end

  # 4. Clear
  expect_frozen_error.call('clear') do
    h.clear
  end

  # 5. In-place merge
  expect_frozen_error.call('merge!') do
    h.merge!(b: 2)
  end

  # 6. Replace
  expect_frozen_error.call('replace') do
    h.replace({})
  end

  # 7. Rehash
  expect_failure_if_artichoke(RuntimeError, 'Expected FrozenError calling: rehash, but no error was raised!') do
    # https://github.com/mruby/mruby/issues/6485
    expect_frozen_error.call('rehash') do
      h.rehash
    end
  end

  # 8. Set a simple default
  expect_frozen_error.call('default=') do
    h.default = :some_value
  end

  # 9. Set a default proc
  expect_frozen_error.call('default_proc=') do
    h.default_proc = proc { |_, _| 'anything' }
  end
end

# Optionally, you can auto-run the spec if this file is loaded as a script:
if $PROGRAM_NAME == __FILE__
  result = spec
  puts "All Hash functional tests passed: #{result}"
end
