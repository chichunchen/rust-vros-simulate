for LEVEL2 in 2200 2300 2400 2500 2600 2700 2800 2900 3000 3100 3200 3300 3400
do
    for SCREEN in 1200 1300 1400 1500 1600 1700 1800 1900 2000
    do
		echo SCREEN: ${SCREEN} LEVEL2: ${LEVEL2}
        cargo run user_viewport_result/Elephant-training-2bpICIClAIg tracedump/elephant.txt object_as_cluster/elephant.json 0.96 20 ${SCREEN} ${SCREEN} ${LEVEL2} 2160 >> elephant_dump.txt &
    done
    wait
done
