// generics

function sum<T extends number | string>(a: T, b: T): T {
  // now since we added both number and string as possible types for T
  // we need to handle both cases inside the function
  if (typeof a === "number" && typeof b === "number") {
    return (a + b) as T; // TypeScript knows this is a number
  } else if (typeof a === "string" && typeof b === "string") {
    return (a + b) as T; // TypeScript knows this is a string
  } else {
    throw new Error(
      "Invalid types: both arguments must be of the same type, either number or string."
    );
  }
}

sum(1, 2); // valid
// sum(1, '2'); // invalid
sum("hi", "uoo"); // valid
// sum(true, false); // invalid
sum(1.0, 2.5); // valid
sum("hello ", "world"); // valid
