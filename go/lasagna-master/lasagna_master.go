package lasagna

// TODO: define the 'PreparationTime()' function
func PreparationTime(layers []string, time int) int {
	if time == 0 {
		time = 2
	}
	return len(layers) * time
}

// TODO: define the 'Quantities()' function
func Quantities(layers []string) (int, float64) {
	countNoodles := 0
	countSauce := 0
	for _, v := range layers {
		if v == "noodles" {
			countNoodles += 1
		}
		if v == "sauce" {
			countSauce += 1
		}
	}
	var expNoodles int = countNoodles * 50
	var expSauce float64 = float64(countSauce) * 0.2
	return expNoodles, expSauce
}

// TODO: define the 'AddSecretIngredient()' function
func AddSecretIngredient(friendsList []string, myList []string) []string {
	myList[len(myList)-1] = friendsList[len(friendsList)-1]
	return myList
}

// TODO: define the 'ScaleRecipe()' function
func ScaleRecipe(input []float64, portions int) []float64 {
	var ret []float64
	for _, i := range input {
		ret = append(ret, i*float64(portions)*0.5)
	}
	return ret
}
