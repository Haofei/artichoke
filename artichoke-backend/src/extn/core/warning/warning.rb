# frozen_string_literal: true

# FIXME: Implement `Warning.warn` method to handle Ruby warnings properly.
# Currently stubbed due to method visibility limitations in the mruby VM. Full
# implementation will resolve duplicate warnings.
#
# See issue: https://github.com/artichoke/artichoke/issues/2844
module Warning
  # FIXME: Delegate warnings to `Warning.warn` instead of directly printing to
  # stderr. This prevents duplicate output and aligns with MRI behavior.
  # Currently blocked by method visibility limitations in the mruby VM, which
  # prevent shadowing `Kernel#warn` with `Warning.warn`.
  #
  # See issue: https://github.com/artichoke/artichoke/issues/2844
  #
  # def warn(message)
  #   $stderr.print(message)
  # end
end
