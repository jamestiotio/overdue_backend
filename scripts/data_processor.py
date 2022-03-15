import json

if __name__ == "__main__":
    total_number_of_materials = 0
    total_materials = {}
    try:
        with open("input_materials.json", "r") as f:
            for line in f:
                try:
                    materials = json.loads(line)
                    for material in materials:
                        if material["name"] not in total_materials:
                            total_materials[str(material["name"])] = material["quantity"]
                        else:
                            total_materials[str(material["name"])] += material["quantity"]

                        total_number_of_materials += material["quantity"]
                except (ValueError, KeyError) as e:
                    print("Error encountered! Please ensure that the input file is in the correct data format.")
                    exit(1)
    except IOError as e:
        print("Error encountered! Please ensure that the input file with the correct name exists.")
        exit(1)

    print("MATERIALS: " + str(total_materials))
    print("TOTAL CONTRIBUTION: " + str(total_number_of_materials))
