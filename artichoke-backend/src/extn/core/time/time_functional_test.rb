# frozen_string_literal: true

def spec
  time_strftime_utf8
  time_strftime_binary
  time_strftime_empty_warning_verbose
  expect_failure_if_artichoke(RuntimeError, /Unexpected warning emitted when verbose mode off, got: /) do
    # Artichoke does not support the `$VERBOSE` global variable.
    # Warnings are unconditionally printed to `$stderr`.
    time_strftime_empty_warning
  end

  true
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

    case error_message
    when Regexp
      unless e.message =~ error_message
        raise "Unexpected error message: #{e.message.inspect}\nExpected: #{error_message.inspect}"
      end
    when String
      unless e.message == error_message
        raise "Unexpected error message: #{e.message.inspect}\nExpected: #{error_message.inspect}"
      end
    else
      raise "Invalid error_message type: #{error_message.class}. Expected String or Regexp."
    end
    # If message matches, we pass
  end
end

def time_strftime_utf8
  t = Time.at(1000, 0, in: 'Z')

  raise unless t.strftime('%c') == 'Thu Jan  1 00:16:40 1970'
  raise unless "%c \xEF".valid_encoding? == false
  raise unless t.strftime("%c \xEF") == "Thu Jan  1 00:16:40 1970 \xEF"
  raise unless t.strftime("%c \xEF").valid_encoding? == false
  raise unless t.strftime("%c \u{1F600}") == 'Thu Jan  1 00:16:40 1970 😀'
  raise unless t.strftime("%c \u{1F600}").length == 26
  raise unless t.strftime('%c 😀') == 'Thu Jan  1 00:16:40 1970 😀'
  raise unless t.strftime('%c 😀').length == 26
end

def time_strftime_binary
  t = Time.at(1000, 0, in: 'Z')

  raise unless t.strftime('%c'.b) == 'Thu Jan  1 00:16:40 1970'
  raise unless "%c \xEF".b.valid_encoding?
  raise unless t.strftime("%c \xEF".b) == "Thu Jan  1 00:16:40 1970 \xEF".b
  raise unless t.strftime("%c \xEF".b).valid_encoding?
  raise unless t.strftime("%c \u{1F600}".b) == "Thu Jan  1 00:16:40 1970 \xF0\x9F\x98\x80".b
  raise unless t.strftime("%c \u{1F600}".b).length == 29
  raise unless t.strftime('%c 😀'.b) == "Thu Jan  1 00:16:40 1970 \xF0\x9F\x98\x80".b
  raise unless t.strftime('%c 😀'.b).length == 29
end

def time_strftime_empty_warning_verbose
  original_verbose = $VERBOSE
  original_stderr = $stderr

  captured_warnings = []
  fake_stderr = Object.new
  fake_stderr.define_singleton_method(:write) { |msg| captured_warnings << msg }

  begin
    $VERBOSE = true
    $stderr = fake_stderr

    Time.now.strftime('')

    unless captured_warnings.any? { |message| message.end_with?("strftime called with empty format string\n") }
      raise "Expected warning was not emitted, got: #{captured_warnings.inspect}"
    end
  ensure
    $VERBOSE = original_verbose
    $stderr = original_stderr
  end
end

def time_strftime_empty_warning
  original_verbose = $VERBOSE
  original_stderr = $stderr

  captured_warnings = []
  fake_stderr = Object.new
  fake_stderr.define_singleton_method(:write) { |msg| captured_warnings << msg }

  begin
    $VERBOSE = false
    $stderr = fake_stderr

    Time.now.strftime('')

    unless captured_warnings.empty?
      raise "Unexpected warning emitted when verbose mode off, got: #{captured_warnings.inspect}"
    end
  ensure
    $VERBOSE = original_verbose
    $stderr = original_stderr
  end
end

if $PROGRAM_NAME == __FILE__
  result = spec
  puts "All Time functional tests passed: #{result}"
end
