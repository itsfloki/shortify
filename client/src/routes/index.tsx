import Form from '~/components/Form'

export default function Home() {
  return (
    <main class="text-center mx-auto text-gray-700 p-4">
      <h1 class="max-6-xs text-6xl text-blue-700 font-thin uppercase my-10">
        Shortify
      </h1>

      <Form />

      <p class="mt-8">
        Visit{' '}
        <a
          href="https://devabhijit.in"
          target="_blank"
          class="text-sky-600 hover:underline"
        >
          devabhijit.in
        </a>{' '}
        to know more about me.
      </p>
    </main>
  )
}
