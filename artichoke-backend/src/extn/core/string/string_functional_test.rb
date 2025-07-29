# frozen_string_literal: true

def spec
  expect_failure_if_artichoke(RuntimeError, "got wrong exception message: undefined method '=~'") do
    # Artichoke raises the wrong exception message for no method errors
    test_string_match_operator
  end
  test_string_element_reference_regexp
  test_string_byteslice
  test_string_byteindex
  test_string_scan
  test_string_unary_minus
  test_string_reverse
  test_string_tr
  test_string_end_with
  test_string_to_i
  test_string_eq

  test_string_concat
  test_string_chomp
  test_string_sub_and_gsub
  test_string_index
  test_string_include_empty

  test_string_swapcase
  test_string_swapcase_bang
  test_swapcase_bang_frozen
  test_string_downcase
  test_string_downcase_bang
  test_downcase_bang_frozen
  test_string_upcase
  test_string_upcase_bang
  test_string_swapcase
  test_upcase_bang_frozen
  test_string_capitalize
  test_string_capitalize_bang
  test_capitalize_bang_frozen

  test_string_initialize_copy_frozen
  test_string_replace_frozen
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

########################################
# Utility for type error checks
########################################

def assert_raise_type_error
  yield
  raise 'Expected TypeError, but no error was raised!'
rescue TypeError
  # pass
end

########################################
# 1. =~ operator
########################################

def test_string_match_operator
  match = "cat o' 9 tails" =~ /\d/
  raise "Expected match=7, got #{match.inspect}" unless match == 7

  begin
    "cat o' 9 tails" =~ 9
    raise 'Expected NoMethodError, but no error was raised!'
  rescue NoMethodError => e
    unless e.message == "undefined method '=~' for an instance of Integer"
      raise "got wrong exception message: #{e.message}"
    end
  end
end

########################################
# 2. Element reference with a Regexp
########################################

def test_string_element_reference_regexp
  # Basic usage
  result = 'hello there'[/[aeiou](.)\1/]
  raise "Expected 'ell', got #{result.inspect}" unless result == 'ell'

  # capture group
  result0 = 'hello there'[/[aeiou](.)\1/, 0]
  raise "Expected 'ell', got #{result0.inspect}" unless result0 == 'ell'

  result1 = 'hello there'[/[aeiou](.)\1/, 1]
  raise "Expected 'l', got #{result1.inspect}" unless result1 == 'l'

  result2 = 'hello there'[/[aeiou](.)\1/, 2]
  raise "Expected nil, got #{result2.inspect}" unless result2.nil?

  # named captures
  vow = 'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'vowel']
  raise "Expected 'e', got #{vow.inspect}" unless vow == 'e'

  non_vow = 'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'non_vowel']
  raise "Expected 'l', got #{non_vow.inspect}" unless non_vow == 'l'
end

########################################
# 3. byteslice
########################################

def test_string_byteslice
  s = 'abcdefghijk' # bytesize == 11
  # scalar
  raise "Expected 'abcdefghijk', got #{s.byteslice(0, 1000).inspect}" unless s.byteslice(0, 1000) == 'abcdefghijk'
  raise "Expected 'fghijk', got #{s.byteslice(5, 1000).inspect}" unless s.byteslice(5, 1000) == 'fghijk'
  raise "Expected nil, got #{s.byteslice(20, 1000).inspect}" unless s.byteslice(20, 1000).nil?
  raise "Expected 'ghijk', got #{s.byteslice(-5, 1000).inspect}" unless s.byteslice(-5, 1000) == 'ghijk'
  raise "Expected nil, got #{s.byteslice(-25, 1000).inspect}" unless s.byteslice(-25, 1000).nil?
  raise "Expected nil, got #{s.byteslice(-25).inspect}" unless s.byteslice(-25).nil?
  raise "Expected 'g', got #{s.byteslice(-5).inspect}" unless s.byteslice(-5) == 'g'
  raise "Expected 'ghijk', got #{s.byteslice(-5, 10).inspect}" unless s.byteslice(-5, 10) == 'ghijk'
  raise "Expected 'a', got #{s.byteslice(0).inspect}" unless s.byteslice(0) == 'a'
  raise "Expected 'c', got #{s.byteslice(2).inspect}" unless s.byteslice(2) == 'c'
  raise "Expected 'abcde', got #{s.byteslice(0, 5).inspect}" unless s.byteslice(0, 5) == 'abcde'
  raise "Expected 'fgh', got #{s.byteslice(5, 3).inspect}" unless s.byteslice(5, 3) == 'fgh'
  raise "Expected nil, got #{s.byteslice(5, -10).inspect}" unless s.byteslice(5, -10).nil?

  # range
  {
    (0..0) => 'a',
    (0..1) => 'ab',
    (0..10) => 'abcdefghijk',
    (1..9) => 'bcdefghij',
    (9..10) => 'jk',
    (9..11) => 'jk',
    (10..10) => 'k',
    (10..11) => 'k',
    (11..11) => '',
    (11..12) => '',
    (1..0) => '',
    (10..0) => '',
    (9..1) => '',
    (10..9) => '',
    (11..9) => '',
    (11..10) => '',
    (-11..0) => 'a',
    (-11..1) => 'ab',
    (-11..10) => 'abcdefghijk',
    (-11..11) => 'abcdefghijk',
    (-10..9) => 'bcdefghij',
    (-2..10) => 'jk',
    (-1..10) => 'k',
    (0..-11) => 'a',
    (0..-10) => 'ab',
    (0..-1) => 'abcdefghijk',
    (1..-2) => 'bcdefghij',
    (9..-1) => 'jk',
    (10..-1) => 'k',
    (-11..-11) => 'a',
    (-11..-10) => 'ab',
    (-11..-1) => 'abcdefghijk',
    (-2..-1) => 'jk',
    (-1..-1) => 'k',
    (-11..-12) => '',
    (-10..-12) => '',
    (-10..-11) => '',
    (-1..-11) => '',
    (-1..-2) => ''
  }.each do |range, expected|
    actual = s.byteslice(range)
    raise "Expected #{expected.inspect} for range #{range}, got #{actual.inspect}" unless actual == expected
  end

  # non-ascii range test
  s = '太贵了!!' # bytesize == 11
  {
    (0..0) => "\xE5",
    (0..1) => "\xE5\xA4",
    (0..10) => '太贵了!!',
    (1..9) => "\xA4\xAA贵了!",
    (9..10) => '!!',
    (9..11) => '!!',
    (10..10) => '!',
    (10..11) => '!',
    (11..11) => '',
    (11..12) => '',
    (-11..0) => "\xE5",
    (-11..1) => "\xE5\xA4",
    (-11..10) => '太贵了!!',
    (-11..11) => '太贵了!!',
    (-10..9) => "\xA4\xAA贵了!",
    (-2..10) => '!!',
    (-1..10) => '!',
    (0..-11) => "\xE5",
    (0..-10) => "\xE5\xA4",
    (0..-1) => '太贵了!!',
    (1..-2) => "\xA4\xAA贵了!",
    (9..-1) => '!!',
    (10..-1) => '!',
    (-11..-11) => "\xE5",
    (-11..-10) => "\xE5\xA4",
    (-11..-1) => '太贵了!!',
    (-2..-1) => '!!',
    (-1..-1) => '!',
    (-11..-12) => '',
    (-10..-12) => '',
    (-10..-11) => '',
    (-1..-11) => '',
    (-1..-2) => ''
  }.each do |range, expected|
    actual = s.byteslice(range)
    unless actual == expected
      raise "Expected #{expected.inspect} for range #{range} in non-ASCII, got #{actual.inspect}"
    end
  end
end

########################################
# 4. byteindex
########################################

def test_string_byteindex
  s = 'foo'
  raise "Expected index=0 for 'f' in 'foo'" unless s.byteindex('f') == 0 # rubocop:disable Style/NumericPredicate

  raise "Expected index=1 for 'o'" unless s.byteindex('o') == 1
  raise "Expected index=1 for 'oo'" unless s.byteindex('oo') == 1
  raise "Expected nil for 'ooo' in 'foo'" unless s.byteindex('ooo').nil?

  raise 'Expected nil for /z/' unless s.byteindex(/z/).nil?
  raise 'Expected index=0 for /f/' unless s.byteindex(/f/) == 0 # rubocop:disable Style/NumericPredicate
  raise 'Expected index=1 for /o/' unless s.byteindex(/o/) == 1
  raise 'Expected index=1 for /oo/' unless s.byteindex(/oo/) == 1
  raise 'Expected nil for /ooo/' unless s.byteindex(/ooo/).nil?

  raise 'Expected nil for /f/ from start=2' unless s.byteindex(/f/, 2).nil?
  raise 'Expected 1 for /o/ from start=1' unless s.byteindex(/o/, 1) == 1
  raise 'Expected 2 for /o/ from start=2' unless s.byteindex(/o/, 2) == 2

  return if 'abcdef'.byteindex(/(c).*(f)/, 2) == 2

  raise "Expected index=2 for /(c).*(f)/, from start=2 in 'abcdef'"
end

########################################
# 5. scan
########################################

def test_string_scan
  s = 'abababa'
  res = s.scan(/./)
  raise "Expected all single chars, got #{res.inspect}" unless res == %w[a b a b a b a]

  res2 = s.scan(/../)
  raise "Expected pairs, got #{res2.inspect}" unless res2 == %w[ab ab ab]

  res3 = s.scan('aba')
  raise "Expected two 'aba's, got #{res3.inspect}" unless res3 == %w[aba aba]

  res4 = s.scan('no no no')
  return if res4 == []

  raise "Expected empty array for 'no no no', got #{res4.inspect}"
end

########################################
# 6. unary minus
########################################

def test_string_unary_minus
  s = -'abababa'
  raise 'Expected a frozen copy, but s was not frozen' unless s.frozen?
  return if s == 'abababa'

  raise "Expected 'abababa' content, got #{s.inspect}"
end

########################################
# 7. reverse
########################################

def test_string_reverse
  return if '再见'.reverse == '见再'

  raise "Expected '见再', got #{'再见'.reverse.inspect}"
end

########################################
# 8. tr
########################################

def test_string_tr
  out = 'abcd'.tr('a-z', 'xxx')
  return if out == 'xxxx'

  raise "Expected 'xxxx', got #{out.inspect}"
end

########################################
# 9. end_with?
########################################

def test_string_end_with
  # Forbids regex in standard Ruby
  assert_raise_type_error { 'abc'.end_with?(/c/) }
  assert_raise_type_error { 'abc'.end_with?('e', 'xyz', /c/) }

  raise 'end with failed when given multiple, misxed type matcher arguments' unless 'abc'.end_with?('e', 'bc', /c/)
  raise "Expected 'abc'.end_with?('bc','abc') => true" unless 'abc'.end_with?('bc', 'abc')
end

########################################
# 10. to_i
########################################

def test_string_to_i
  # whitespace
  trimmed = "\x0B\n\r\t\x0C 123".to_i
  raise "Expected 123, got #{trimmed.inspect}" unless trimmed == 123

  # underscores
  part = '1__23'.to_i
  return if part == 1

  raise "Expected '1__23'.to_i => 1, got #{part.inspect}"
end

########################################
# 11. eq?
########################################

def test_string_eq
  # distinct enc
  left = '太贵了!!'
  right = '太贵了!!'.b
  raise 'Expected left != right for different enc, got eq' unless left != right
  raise 'Expected right != left for different enc, got eq' unless right != left

  a = "\x7f"
  b = a.b
  raise 'Expected \\x7f and \\x7f.b => eq' unless a == b
  raise 'Expected eq in reverse' unless b == a

  c = "\x80"
  d = c.b
  raise 'Expected c != d for real negative in default MRI, got eq' if c == d
  return unless d == c

  raise 'Expected d != c, got eq'
end

########################################
# 12. concat
########################################

def test_string_concat
  s = +'hello'
  result = s.concat(' world')
  raise "Expected 'hello world' after concat, got #{s.inspect}" unless s == 'hello world'
  raise 'concat did not return self' unless result.equal?(s)

  # numeric arg => char
  result = s.concat(33) # '!'
  raise "Expected 'hello world!', got #{s.inspect}" unless s == 'hello world!'
  raise 'concat did not return self' unless result.equal?(s)

  # If we freeze s, we expect a FrozenError on concat:
  s.freeze
  begin
    s.concat('again')
    raise 'Expected FrozenError, but none was raised!'
  rescue FrozenError
    # pass
  end
end

########################################
# 13. chomp
########################################

def test_string_chomp
  #
  # Scenario 1: line_sep == "\n" (the default)
  # (Removes one or two trailing characters if they are "\r", "\n", or "\r\n"—but not "\n\r")
  #

  result1 = "abc\r".chomp
  raise "Expected 'abc', got #{result1.inspect}" unless result1 == 'abc'

  result2 = "abc\n".chomp
  raise "Expected 'abc', got #{result2.inspect}" unless result2 == 'abc'

  result3 = "abc\r\n".chomp
  raise "Expected 'abc', got #{result3.inspect}" unless result3 == 'abc'

  result4 = "abc\n\r".chomp
  raise "Expected \"abc\\n\", got #{result4.inspect}" unless result4 == "abc\n"

  result5 = "тест\r\n".chomp
  raise "Expected 'тест', got #{result5.inspect}" unless result5 == 'тест'

  result6 = "こんにちは\r\n".chomp
  raise "Expected 'こんにちは', got #{result6.inspect}" unless result6 == 'こんにちは'

  #
  # Scenario 2: line_sep == '' (empty string)
  # (Removes multiple trailing occurrences of "\n" or "\r\n", but not "\r" or "\n\r")
  #

  result7 = "abc\n\n\n".chomp('')
  raise "Expected 'abc', got #{result7.inspect}" unless result7 == 'abc'

  result8 = "abc\r\n\r\n\r\n".chomp('')
  raise "Expected 'abc', got #{result8.inspect}" unless result8 == 'abc'

  result9 = "abc\n\n\r\n\r\n\n\n".chomp('')
  raise "Expected 'abc', got #{result9.inspect}" unless result9 == 'abc'

  result10 = "abc\n\r\n\r\n\r".chomp('')
  raise "Expected \"abc\\n\\r\\n\\r\\n\\r\", got #{result10.inspect}" unless result10 == "abc\n\r\n\r\n\r"

  result11 = "abc\r\r\r".chomp('')
  raise "Expected \"abc\\r\\r\\r\", got #{result11.inspect}" unless result11 == "abc\r\r\r"

  #
  # Scenario 3: line_sep is neither "\n" nor ''
  # Removes exactly one trailing occurrence of the given separator if there is one
  #

  result12 = 'abcd'.chomp('d')
  raise "Expected 'abc', got #{result12.inspect}" unless result12 == 'abc'

  result13 = 'abcdd'.chomp('d')
  raise "Expected 'abcd', got #{result13.inspect}" unless result13 == 'abcd'
end

########################################
# 14. sub and gsub
########################################

def test_string_sub_and_gsub
  s = 'cats and dogs'
  subbed = s.sub(/dogs/, 'ponies')
  raise "Expected 'cats and ponies', got #{subbed.inspect}" unless subbed == 'cats and ponies'
  raise 'Expected original unchanged' unless s == 'cats and dogs'

  gsubbed = s.gsub(/s/, 'z')
  return if gsubbed == 'catz and dogz'

  raise "Expected 'catz and dogz', got #{gsubbed.inspect}"
end

########################################
# 15. index
########################################

def test_string_index
  s = 'abcdefgh'
  idx = s.index('cde')
  raise "Expected index=2 for 'cde'" unless idx == 2

  # returns nil if not found
  idx2 = s.index('zzz')
  raise "Expected nil, got #{idx2.inspect}" unless idx2.nil?

  # with start pos
  idx3 = s.index('c', 3)
  return if idx3.nil?

  raise "Expected nil for 'c' from pos=3"
end

########################################
# 16. empty? and include?
########################################

def test_string_include_empty
  s = ''
  raise 'Expected empty string to be empty?' unless s.empty?

  s2 = 'hello'
  raise "Expected 'hello' not empty" if s2.empty?

  # include?
  raise "Expected 'hello'.include?('ell') => true" unless s2.include?('ell')
  return unless s2.include?('zzz')

  raise "Expected false for .include?('zzz')"
end

########################################
# 17. swapcase
########################################

def test_string_swapcase
  s = 'AbCd'
  swapped = s.swapcase
  raise "Expected 'aBcD', got #{swapped.inspect}" unless swapped == 'aBcD'
  raise "Expected original unchanged, got #{s.inspect}" unless s == 'AbCd'

  s = +'Hello World!' # => "Hello World!"
  result = s.swapcase
  expected = 'hELLO wORLD!'
  return if result == expected

  raise "FAIL: `#{s}.swapcase` => #{result.inspect}, expected #{expected.inspect}"
end

def test_string_swapcase_bang
  s = +'Hello World!'
  changed = s.swapcase!
  expected = 'hELLO wORLD!'
  raise "FAIL: s.swapcase! => #{changed.inspect}, expected #{expected.inspect}" unless changed == expected
  raise "FAIL: string changed in place => #{s.inspect}, expected #{expected.inspect}" unless s == expected

  # If no changes are made, returns nil
  s = +'123'
  unchanged = s.swapcase!
  raise "FAIL: repeated swapcase! => expected nil, got #{unchanged.inspect}" unless unchanged.nil?

  # Also doc says: `"".swapcase! => nil`
  empty_str = +''
  none = empty_str.swapcase!
  return if none.nil?

  raise "FAIL: ''.swapcase! => expected nil, got #{none.inspect}"
end

def test_swapcase_bang_frozen
  s = 'HeLLo'.freeze # rubocop:disable Style/RedundantFreeze
  begin
    s.swapcase!
    raise 'Expected FrozenError for swapcase! on a frozen string, but got none!'
  rescue FrozenError
    # pass
  end
end

########################################
# 18. downcase
########################################

def test_string_downcase
  s = +'Hello World!' # => "Hello World!"
  result = s.downcase
  expected = 'hello world!'
  return if result == expected

  raise "FAIL: `#{s}.downcase` => #{result.inspect}, expected #{expected.inspect}"
end

def test_string_downcase_bang
  s = +'Hello World!'
  changed = s.downcase!
  raise "FAIL: `s.downcase!` => #{changed.inspect}, expected 'hello world!'" unless changed == 'hello world!'
  raise "FAIL: string changed in place => #{s.inspect}, expected 'hello world!'" unless s == 'hello world!'

  # If no changes were made, returns nil
  unchanged = s.downcase!
  return if unchanged.nil?

  raise "FAIL: repeated downcase! => expected nil, got #{unchanged.inspect}"
end

def test_downcase_bang_frozen
  s = 'HELLO'.freeze # rubocop:disable Style/RedundantFreeze
  begin
    s.downcase!
    raise 'Expected FrozenError for downcase! on a frozen string, but got none!'
  rescue FrozenError
    # pass
  end
end

########################################
# 19. upcase
########################################

def test_string_upcase
  s = 'Hello World!'
  result = s.upcase
  expected = 'HELLO WORLD!'
  return if result == expected

  raise "FAIL: `#{s}.upcase` => #{result.inspect}, expected #{expected.inspect}"
end

def test_string_upcase_bang
  s = +'Hello World!'
  changed = s.upcase!
  raise "FAIL: s.upcase! => #{changed.inspect}, expected 'HELLO WORLD!'" unless changed == 'HELLO WORLD!'
  raise "FAIL: string changed in place => #{s.inspect}, expected 'HELLO WORLD!'" unless s == 'HELLO WORLD!'

  unchanged = s.upcase!
  return if unchanged.nil?

  raise "FAIL: repeated upcase! => expected nil, got #{unchanged.inspect}"
end

def test_upcase_bang_frozen
  s = 'hello'.freeze # rubocop:disable Style/RedundantFreeze
  begin
    s.upcase!
    raise 'Expected FrozenError for upcase! on a frozen string, but got none!'
  rescue FrozenError
    # pass
  end
end

########################################
# 20. capitalize
########################################

def test_string_capitalize
  s = 'hello World!' # => "hello World!"
  result = s.capitalize
  expected = 'Hello world!'
  return if result == expected

  raise "FAIL: `#{s}.capitalize` => #{result.inspect}, expected #{expected.inspect}"
end

def test_string_capitalize_bang
  s = +'hello World!' # => "hello World!"
  # after capitalize! => "Hello world!"
  changed = s.capitalize!
  raise "FAIL: s.capitalize! => #{changed.inspect}, expected 'Hello world!'" unless changed == 'Hello world!'
  raise "FAIL: string changed in place => #{s.inspect}, expected 'Hello world!'" unless s == 'Hello world!'

  # if no changes are made, returns nil
  unchanged = s.capitalize!
  return if unchanged.nil?

  raise "FAIL: repeated capitalize! => expected nil, got #{unchanged.inspect}"
end

def test_capitalize_bang_frozen
  s = 'heLLo'.freeze # rubocop:disable Style/RedundantFreeze
  begin
    s.capitalize!
    raise 'Expected FrozenError for capitalize! on a frozen string, but got none!'
  rescue FrozenError
    # pass
  end
end

########################################
# 20. frozen string tests
########################################

def test_string_initialize_copy_frozen
  # Create two strings: a frozen receiver, and another string we'll attempt
  # to copy into the receiver.
  frozen_receiver = 'hello'.freeze # rubocop:disable Style/RedundantFreeze
  source = 'world'

  begin
    # Since `initialize_copy` is private, we must call it via `send`.
    frozen_receiver.send(:initialize_copy, source)
    raise 'Expected FrozenError for initialize_copy on a frozen string, but got none!'
  rescue FrozenError
    # pass
  end
end

def test_string_replace_frozen
  # `String#replace` is the public API that eventually calls initialize_copy
  # in many Ruby implementations. This should also raise FrozenError.
  frozen_receiver = 'hello'.freeze # rubocop:disable Style/RedundantFreeze
  source = 'world'

  begin
    frozen_receiver.replace(source)
    raise 'Expected FrozenError for replace on a frozen string, but got none!'
  rescue FrozenError
    # pass
  end
end

if $PROGRAM_NAME == __FILE__
  result = spec
  puts "All String functional tests passed: #{result}"
end
