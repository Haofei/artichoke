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

  # If all tests pass, return a truthy value
  true
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

# Optionally, you can auto-run the spec if this file is loaded as a script:
if $PROGRAM_NAME == __FILE__
  result = spec
  puts "All Hash functional tests passed: #{result}"
end
