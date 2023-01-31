// Package weather This is weather package.
package weather

// CurrentCondition is the weather condition.
var CurrentCondition string

// CurrentLocation is the city.
var CurrentLocation string

// Forecast weather for city.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
