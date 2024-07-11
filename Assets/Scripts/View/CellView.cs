using System;
using UnityEditor;
using UnityEngine;
using UnityEngine.EventSystems;
using View;

namespace View
{
    public enum CellViewType
    {
        X,
        O,
        Empty
    }

    public class CellView : MonoBehaviour, IPointerClickHandler
    {
        [SerializeField] public Vector2Int position;
        [SerializeField] private GameObject xObject;
        [SerializeField] private GameObject oObject;
        public Action onClickAction;

        public void OnPointerClick(PointerEventData eventData)
        {
            OnClick();
        }

        public void Init(Vector2Int pos, CellViewType cellViewType)
        {
            position = pos;
            switch (cellViewType)
            {
                case CellViewType.X:
                    ToX();
                    break;
                case CellViewType.O:
                    ToO();
                    break;
                case CellViewType.Empty:
                    ToEmpty();
                    break;
                default:
                    throw new ArgumentOutOfRangeException();
            }
        }

        internal void OnClick()
        {
            onClickAction.Invoke();
            Debug.Log($"CellView.OnClick: {position}");
        }

        public void ToX()
        {
            Debug.Log($"CellView.ToX: {position}");
            xObject.SetActive(true);
            oObject.SetActive(false);
        }

        public void ToO()
        {
            Debug.Log($"CellView.ToO: {position}");
            xObject.SetActive(false);
            oObject.SetActive(true);
        }

        public void ToEmpty()
        {
            Debug.Log($"CellView.ToEmpty: {position}");
            xObject.SetActive(false);
            oObject.SetActive(false);
        }
    }
}

#if UNITY_EDITOR
[CustomEditor(typeof(CellView))]
public class CellViewEditor : Editor
{
    public override void OnInspectorGUI()
    {
        base.OnInspectorGUI();
        var cellView = (CellView)target;
        if (GUILayout.Button("ToX")) cellView.ToX();
        if (GUILayout.Button("ToO")) cellView.ToO();
        if (GUILayout.Button("ToEmpty")) cellView.ToEmpty();
        if (GUILayout.Button("OnClick")) cellView.OnClick();
    }
}


#endif