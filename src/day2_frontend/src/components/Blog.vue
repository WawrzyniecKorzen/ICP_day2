<template>
    <div>
        <h2 class="text-blue-600">Wpisy na bloga</h2>
        <div class="w-100 flex flex-row-reverse">
            <button @click="pobierzWpisy" class="bg-blue-600 rounded text-white p-4">Pobierz wpisy</button>
        </div>
        <div class="grid mx-6 gap-4 my-4">

            <div v-for="wpis in wpisy" class="drop-shadow-xl bg-stone-300 p-4">
                <p>{{wpis}}</p>
            </div>
        </div>
        
        <div class="flex justify-center flex-col mx-6 gap-4 my-4">
            <input v-model="nowyBlog" class="border-2 border-blue-600 p-4 w-" type="text" >
            <button @click="dodajWpis" class="bg-blue-600 rounded text-white p-4">Dodaj wpis</button>
        </div>
        
    </div>
</template>

<script>
import { day2_backend } from 'declarations/day2_backend/index';

export default
{
    data()
    {
        return {
            wpisy: [],
            nowyBlog: ""
        }
    },
    methods:
    {
        async pobierzWpisy()
        {
            this.wpisy = await day2_backend.odczytaj_wpisy();
        },
        async dodajWpis()
        {
            await day2_backend.dodaj_wpis(this.nowyBlog);
        }
    },
    async mounted()
    {
        this.pobierzWpisy();
    }
}
</script>