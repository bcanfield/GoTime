import React from "react";
import { useForm } from "@tanstack/react-form";
import type { AnyFieldApi } from "@tanstack/react-form";
import { useSpacetime } from "../../providers/spacetime-context";

type FormValues = {
  boardSize: number;
};

type GameCreationFormModalProps = {
  open: boolean;
  onClose: () => void;
};

function FieldInfo({ field }: { field: AnyFieldApi }) {
  return (
    <>
      {field.state.meta.isTouched && field.state.meta.errors.length > 0 && (
        <div className="text-error text-sm mt-1">
          {field.state.meta.errors.join(", ")}
        </div>
      )}
    </>
  );
}

export default function GameCreationFormModal({
  open,
  onClose,
}: GameCreationFormModalProps) {
  const { conn } = useSpacetime();

  const handleCreateGame = async ({ values }: { values: FormValues }) => {
    console.log({ values });
    // For demonstration, you can optionally pass a handicap (e.g., 2)
    conn?.reducers.createGame(values.boardSize, 0); // 9x9 game without handicap.
  };
  const form = useForm({
    defaultValues: {
      playerName: "",
      boardSize: 9,
    },
    onSubmit: async ({ value }) => {
      handleCreateGame({ values: value });
      onClose();
    },
  });

  return (
    <dialog open={open} className="modal modal-bottom sm:modal-middle">
      <div className="modal-box">
        <h3 className="font-bold text-lg">Create a New Game</h3>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
            form.handleSubmit();
          }}
          className="space-y-4 mt-4"
        >
          <form.Field name="boardSize">
            {(field) => (
              <label className="form-control w-full">
                <span className="label-text">Board Size</span>
                <select
                  id={field.name}
                  className="select select-bordered"
                  value={field.state.value}
                  onChange={(e) => field.handleChange(Number(e.target.value))}
                >
                  {[9, 13, 19].map((size) => (
                    <option key={size} value={size}>
                      {size} x {size}
                    </option>
                  ))}
                </select>
                <FieldInfo field={field} />
              </label>
            )}
          </form.Field>

          <form.Subscribe
            selector={(state) => [state.canSubmit, state.isSubmitting]}
          >
            {([canSubmit, isSubmitting]) => (
              <div className="modal-action">
                <button
                  type="button"
                  className="btn btn-ghost"
                  onClick={onClose}
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="btn btn-primary"
                  disabled={!canSubmit}
                >
                  {isSubmitting ? "Creating..." : "Create Game"}
                </button>
              </div>
            )}
          </form.Subscribe>
        </form>
      </div>
    </dialog>
  );
}
