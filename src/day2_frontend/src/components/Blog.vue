<template>
    <div>
        <h2 class="text-blue-600">Wpisy na bloga</h2>
        <button @click="pobierzWpisy">Pobierz wpisy</button>
        <div v-for="wpis in wpisy">
            <p>{{wpis}}</p>
        </div>

        <input v-model="nowyBlog" type="text">
        <button @click="dodajWpis">Dodaj wpis</button>
        {{ nowyBlog }}
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