export default { 
  extends: ['@commitlint/config-conventional'],
  rules: {
    // Disable max line length check for commit body to allow more detailed descriptions
    // Setting the first parameter to 0 disables the rule entirely
    "body-max-line-length": [0, "always", 150]
  }
};
