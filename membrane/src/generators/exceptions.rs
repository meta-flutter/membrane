pub fn create_exceptions() -> String {
  r#"// AUTO GENERATED FILE, DO NOT EDIT
//
// Generated by `membrane`
abstract class MembraneException implements Exception {
  final String? message;

  const MembraneException(this.message);

  @override
  String toString() {
    {
      return (message == null) ? "$this" : "$this: $message";
    }
  }
}

class CancellationFailedException extends MembraneException {
  const CancellationFailedException([String? message]) : super(message);
}

class MemoryFreeFailedException extends MembraneException {
  const MemoryFreeFailedException([String? message]) : super(message);
}

class RustPanicException extends MembraneException {
  const RustPanicException([String? message]) : super(message);
}

class UnknownResponseVariantException extends MembraneException {
  const UnknownResponseVariantException([String? message]) : super(message);
}
"#
  .to_string()
}
