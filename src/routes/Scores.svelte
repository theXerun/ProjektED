<script lang="ts">
    export let matrix1: Array<number>
    export let matrix2: Array<number>
    export let decimalPoint: number
    
    let acc1 = accuracy(matrix1);
    let sens1 = sensitivity(matrix1);
    let spec1 = specifity(matrix1);
    let prec1 = precision(matrix1);
    let f11 = f1(matrix1);

    let acc2 = accuracy(matrix2);
    let sens2 = sensitivity(matrix2);
    let spec2 = specifity(matrix2);
    let prec2 = precision(matrix2);
    let f12 = f1(matrix2);

    function accuracy(a: Array<number>) {
		return (a[0]+a[3])/(a[0]+a[1]+a[2]+a[3])
	}

	function sensitivity(a: Array<number>) {
		return (a[3])/(a[2]+a[3])
	}

	function specifity(a: Array<number>) {
		return a[0]/(a[0]+a[1])
	}

	function precision(a: Array<number>) {
		return a[3]/(a[1]+a[3])
	}

    function f1(a: Array<number>) {
        return 2 * (sensitivity(a) * precision(a))/(sensitivity(a)+precision(a))
    }
</script>

<div>
    <p class={(acc1>acc2) ? "better" : "worse"}>Trafność: {acc1.toFixed(decimalPoint)}</p>
    <p class={(sens1>sens2) ? "better" : "worse"}>Czułość: {sens1.toFixed(decimalPoint)}</p>
    <p class={(spec1>spec2) ? "better" : "worse"}>Swoistość: {spec1.toFixed(decimalPoint)}</p>
    <p class={(prec1>prec2) ? "better" : "worse"}>Precyzja: {prec1.toFixed(decimalPoint)}</p>
    <p class={(f11>f12) ? "better" : "worse"}>Wskaźnik F1: {f11.toFixed(decimalPoint)}</p>
</div>

<style>
	.better {
		color: green;
	}
    .worse {
        color: red;
    }
</style>